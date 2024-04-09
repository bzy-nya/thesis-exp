use crate::dep::DependencyGraph;
use crate::sat::SAT;

const E : f64 = std::f64::consts::E;

#[derive(Debug)]
pub enum PredictedResult {
    Invalid,
    UpperBound(f64),
}

pub fn symmertric_lll_checker(dep: &DependencyGraph) -> PredictedResult {
    if E * dep.max_p * (dep.max_d as f64) < 1.0 {
        let a = dep.max_p * (1.0 + 1.0 / dep.max_d as f64).powi(dep.max_d as i32);
        PredictedResult::UpperBound( 
            (dep.n as f64 / dep.max_d as f64)
            .min(dep.n as f64 * a / (1.0 - a)) 
        )
    } else {
        PredictedResult::Invalid
    }
}

pub fn shearers_bound_checker(dep: &DependencyGraph) -> PredictedResult {
    // O(2^n * n) by using fmt

    assert!( dep.n <= 20 );

    let n = dep.n;

    let mut q = crate::new_vector(1 << n, 0.0);
    let mut vis = crate::new_vector(n, 0usize);

    fn dfs(
        q: &mut Vec<f64>, 
        vis: &mut Vec<usize>, 
        dep: &DependencyGraph,
        n: usize,
        x: usize, pt: usize, pre: f64) 
    {
        q[x] = pre;

        for npt in pt..n { if vis[npt] == 0 {
            for d in dep.get_gamma_plus(pt)
              { vis[d] += 1; }

            dfs( q, vis, dep, n, x | (1 << npt), npt + 1, pre * dep.get_p(npt) );

            for d in dep.get_gamma_plus(pt)
              { vis[d] -= 1; }
        } }
    }

    dfs(&mut q, &mut vis, dep, n, 0, 0, 1.0);

    // fmt
    for i in 0..n {
        for j in 0..1 << n {
            if ((j >> i) & 1) == 0 {
                q[j] -= q[j | (1 << i)];
            }
        }
    }

    if crate::min_f64(&q) < 0.0 {
        PredictedResult::Invalid
    } else {
        PredictedResult::UpperBound(
            (0..n).map(|i| q[1 << i] ).sum::<f64>() / q[0]
        )
    }
}

#[cfg(test)]
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
