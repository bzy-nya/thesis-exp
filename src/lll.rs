use crate::dep::DependencyGraph;
use crate::sat::SAT;

const E : f64 = std::f64::consts::E;

#[derive(Debug)]
pub enum PredictedResult {
    Invalid,
    UpperBound(f64),
}

pub fn symmertric_lll_checker(dep: &DependencyGraph) -> PredictedResult {
    if E * dep.max_p * ((dep.max_d + 1) as f64) < 1.0 {
        PredictedResult::UpperBound( dep.n as f64 / dep.max_p )
    } else {
        PredictedResult::Invalid
    }
}

pub fn shearers_bound_checker(dep: &DependencyGraph) -> PredictedResult {
    todo!()
}

pub fn correlation_decay_checker(dep: &DependencyGraph) -> PredictedResult {
    todo!()
}

pub fn satisfiability_checker(sat: &SAT) -> PredictedResult {
    use crate::random_space::random_space_of_nbits;

    let m = sat.variable_count();

    let solution: usize = random_space_of_nbits(m)
        .map(|r| {
            let violated = sat.check_violated(&r.collect());
            ( violated.iter().map(|x| *x as usize).sum::<usize>() == 0 ) as usize
    } ).sum();
    
    if solution == 0
      { PredictedResult::Invalid }
    else 
      { PredictedResult::UpperBound( (1 << m) as f64 / solution as f64 ) }
}
