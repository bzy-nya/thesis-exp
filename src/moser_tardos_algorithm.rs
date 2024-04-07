use crate::sat::SAT;
use crate::random_space::RandomSpace;
use crate::dep::DependencyGraph;
use std::collections::BTreeSet;

#[derive(Debug)]
pub enum ExecuteResult {
    RandomSpaceExceed,
    Terminal {
        step: usize
    },
    Running {
        resampled_clause: usize
    }
}

pub struct AlgorithmSimulator<R: RandomSpace> {
    sat:              SAT,
    random_space:     R,
    violated_clause:  BTreeSet<usize>,
    dependency_graph: DependencyGraph,
    varible:          Vec<bool>,
    step:             usize
}

impl<R> AlgorithmSimulator<R> 
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

    pub fn run_init(&mut self) {
        for v in self.varible.iter_mut() {
            *v = self.random_space.fetch_random_bit().unwrap();
        }
        for (id, &b) in self.sat.check_violated(&self.varible).iter().enumerate() {
            if b { self.violated_clause.insert(id); }
        }
    }
    
    pub fn run_next_step(&mut self) -> ExecuteResult {
        match self.violated_clause.pop_first() {
            None => ExecuteResult::Terminal { step: self.step },
            Some(id) => {
                for v in self.sat.get_clause(id).collect_varible() {
                    let new_value = self.random_space.fetch_random_bit();
                    if new_value == None { return ExecuteResult::RandomSpaceExceed; }
                    self.varible[v - 1] = new_value.unwrap();
                }
                for c in self.dependency_graph.get_dependency(id) {
                    if self.sat.get_clause(c).check_violated(&self.varible) {
                        self.violated_clause.insert(c);
                    }
                }
                if self.sat.get_clause(id).check_violated(&self.varible) {
                    self.violated_clause.insert(id);
                }
                self.step += 1;
                ExecuteResult::Running { resampled_clause: id }
            }
        }
    }

    pub fn run_until_terminal(&mut self) -> ExecuteResult {
        loop {
            match self.run_next_step() {
                ExecuteResult::RandomSpaceExceed => { return ExecuteResult::RandomSpaceExceed; },
                ExecuteResult::Running { .. } => {},
                ExecuteResult::Terminal { step } => { return  ExecuteResult::Terminal { step: step } ;}
            }
        }
    }

    pub fn get_varible(&self) -> &Vec<bool> {
        &self.varible
    }
}

impl<R> Iterator for AlgorithmSimulator<R> 
    where R : RandomSpace
{
    type Item = ExecuteResult;
    fn next(&mut self) -> Option<Self::Item> {
        match self.run_next_step() {
            ExecuteResult::Running { resampled_clause } 
                => Some(ExecuteResult::Running { resampled_clause }),
            _ => None
        }
    }
}