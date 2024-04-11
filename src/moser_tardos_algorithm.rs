use crate::sat::SAT;
use crate::random_space::{random_random_space, InfiniteRandomSpace, RandomSpace};
use crate::dep::DependencyGraph;
use std::collections::BTreeSet;

#[derive(Debug)]
pub enum ExecuteResult<ResamplingType> {
    RandomSpaceExceed,
    Terminal {
        step: usize
    },
    Running {
        resampled_clause: ResamplingType
    }
}

pub trait AlgorithmSimulator<R: RandomSpace> {
    type ResamplingType;
    fn new(sat: SAT, random_space: R) -> Self;
    fn run_init(&mut self);
    fn run_next_step(&mut self) -> ExecuteResult<Self::ResamplingType>;
    fn run_until_terminal(&mut self) -> ExecuteResult<Self::ResamplingType> {
        loop {
            match self.run_next_step() {
                ExecuteResult::RandomSpaceExceed => { return ExecuteResult::RandomSpaceExceed; },
                ExecuteResult::Running { .. } => {},
                ExecuteResult::Terminal { step } => { return  ExecuteResult::Terminal { step: step } ;}
            }
        }
    }
}

pub struct MTsAlgorithmSimulator<R: RandomSpace> {
    sat:              SAT,
    random_space:     R,
    violated_clause:  BTreeSet<usize>,
    dependency_graph: DependencyGraph,
    varible:          Vec<bool>,
    step:             usize
}

fn resampling<R>(var: &mut Vec<bool>, random_space: &mut R, list: Vec<usize> ) -> Result<(),()> 
    where R: RandomSpace
{
    for v in list {
        let new_value = random_space.fetch_random_bit();
        if new_value == None { return Err(()); }
        var[v - 1] = new_value.unwrap();
    }
    
    Ok(())
}

impl<R> MTsAlgorithmSimulator<R> 
    where R: RandomSpace
{
    #[cfg(test)]
    pub fn get_varible(&self) -> &Vec<bool> {
        &self.varible
    }
}

impl<R> AlgorithmSimulator<R> for MTsAlgorithmSimulator<R> 
    where R: RandomSpace
{
    type ResamplingType = usize;
    fn new(sat: SAT, random_space: R) -> Self {
        Self {
            dependency_graph: DependencyGraph::form_sat(&sat),
            varible: crate::new_vector( sat.variable_count(), false ),
            violated_clause: BTreeSet::new(),
            step: 0,
            sat, random_space
        }
    }

    fn run_init(&mut self) {
        self.step = 0;

        for v in self.varible.iter_mut() {
            *v = self.random_space.fetch_random_bit().unwrap();
        }
        for (id, &b) in self.sat.check_violated(&self.varible).iter().enumerate() {
            if b { self.violated_clause.insert(id); }
        }
    }
    
    fn run_next_step(&mut self) -> ExecuteResult<usize> {
        match self.violated_clause.pop_first() {
            None => ExecuteResult::Terminal { step: self.step },
            Some(id) => {
                match resampling(
                    &mut self.varible, 
                    &mut self.random_space, 
                    self.sat.get_clause(id).collect_varible()) 
                {
                    Err(_) => ExecuteResult::RandomSpaceExceed,
                    Ok(_) => {
                        for c in self.dependency_graph.get_gamma_plus(id) {
                            if self.sat.get_clause(c).check_violated(&self.varible) {
                                self.violated_clause.insert(c);
                            }
                        }
                        self.step += 1;
                        ExecuteResult::Running { resampled_clause: id }
                    }
                }
                
            }
        }
    }
}

impl<R> Iterator for MTsAlgorithmSimulator<R> 
    where R : RandomSpace
{
    type Item = ExecuteResult<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.run_next_step() {
            ExecuteResult::Running { resampled_clause } 
                => Some(ExecuteResult::Running { resampled_clause }),
            _ => None
        }
    }
}

pub struct NewAlgorithmSimulator<R>
    where R: RandomSpace 
{
    inner: MTsAlgorithmSimulator<R>
}

impl<R> AlgorithmSimulator<R> for NewAlgorithmSimulator<R> 
    where R: RandomSpace
{
    type ResamplingType = Vec<usize>;

    fn new(sat: SAT, random_space: R) -> Self {
        Self {
            inner: MTsAlgorithmSimulator::new(sat, random_space)
        }
    }

    fn run_init(&mut self) {
        self.inner.run_init()
    }
    
    fn run_next_step(&mut self) -> ExecuteResult<Self::ResamplingType> {
        let inner = &mut self.inner;

        match inner.violated_clause.len() {
            0 => ExecuteResult::Terminal { step: inner.step },
            _ => {
                let violated = inner.violated_clause.clone();
                let mut list = BTreeSet::new();

                inner.violated_clause.clear();

                for &c in &violated {
                    for v in inner.sat.get_clause(c).collect_varible() {
                        list.insert(v);
                    }
                }

                match resampling(
                    &mut inner.varible, 
                    &mut inner.random_space, 
                    list.into_iter().collect() 
                ) {
                    Err(_) => ExecuteResult::RandomSpaceExceed,
                    Ok(_) => {
                        for &c in &violated {
                            for cc in inner.dependency_graph.get_gamma_plus(c) {
                                if inner.sat.get_clause(cc).check_violated(&inner.varible) {
                                    inner.violated_clause.insert(cc);
                                }
                            }
                        }
                        self.inner.step += 1;
                        ExecuteResult::Running { resampled_clause: violated.into_iter().collect() }
                    }
                }
                
            }
        }
    }
}

impl<R> Iterator for NewAlgorithmSimulator<R> 
    where R : RandomSpace
{
    type Item = ExecuteResult<Vec<usize>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.run_next_step() {
            ExecuteResult::Running { resampled_clause } 
                => Some(ExecuteResult::Running { resampled_clause }),
            _ => None
        }
    }
}

#[derive(Debug)]
pub enum BenchResult {
    Failed,
    Success {
        mean: f64,
        c99: f64,
    }
}

pub fn bench_algorithm<A>(sat: &SAT, n: usize) -> BenchResult 
    where A: AlgorithmSimulator<InfiniteRandomSpace>
{
    let mut sand_box = A::new(
        sat.clone(), 
        random_random_space()
    );

    let mut results = Vec::new();
    
    const LIMITS: usize = 10000000;

    sand_box.run_init();
    for _ in 0..LIMITS {
        sand_box.run_next_step();
    }
    match sand_box.run_next_step() {
        ExecuteResult::Running { .. } => { return BenchResult::Failed; }
        _ => {}
    }

    for _ in 0..n {
        sand_box.run_init();
        match sand_box.run_until_terminal() {
            ExecuteResult::Terminal { step } => { results.push(step as i32); }
            _ => { return BenchResult::Failed; }
        }
    }

    BenchResult::Success { 
        mean: crate::mean(results.as_slice() ).unwrap() as f64, 
        c99: 2.58 * crate::std_deviation(results.as_slice() ).unwrap() as f64 / (n as f64).sqrt()
    }
}

impl std::fmt::Display for BenchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::Failed => { write!(f, "N/A, N/A") }
            &Self::Success { mean, c99 } => {
                write!(f, "{:.3},{:.3}", mean, mean + c99 )
            }
        }
    }
}