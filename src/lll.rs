use crate::dep::DependencyGraph;

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
