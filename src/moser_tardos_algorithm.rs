use crate::sat::SAT;
use crate::random_space::RandomSpace;
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
    pub fn new(sat: SAT, random_space: R) -> Self {
        Self {
            dependency_graph: DependencyGraph::form_sat(&sat),
            varible: crate::new_vector( sat.variable_count(), false ),
            violated_clause: BTreeSet::new(),
            step: 0,
            sat, random_space
        }
    }

    #[cfg(test)]
    pub fn get_varible(&self) -> &Vec<bool> {
        &self.varible
    }
}

impl<R> AlgorithmSimulator<R> for MTsAlgorithmSimulator<R> 
    where R: RandomSpace
{
    type ResamplingType = usize;
    fn run_init(&mut self) {
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

impl<R> NewAlgorithmSimulator<R> 
    where R: RandomSpace
{
    pub fn new(sat: SAT, random_space: R) -> Self {
        Self {
            inner: MTsAlgorithmSimulator::new(sat, random_space)
        }
    }
}

impl<R> AlgorithmSimulator<R> for NewAlgorithmSimulator<R> 
    where R: RandomSpace
{
    type ResamplingType = Vec<usize>;
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

