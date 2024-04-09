mod utility;
use utility::*;

mod sat;
mod dep;
mod lll;
mod r#match;
mod random_space;
mod moser_tardos_algorithm;

#[cfg(test)]
mod tests;

fn main() {
    let sat = [[-1, 2, 4], [1, 3, 5], [2, 4, 5], [-3, 5, 6]];
    let sat = sat::SAT::from_array(sat);
    let mat = r#match::Match::from_sat_greedy(&sat);
    let dep = dep::DependencyGraph::form_sat(&sat);

    println!( "lll predict     : {:?}", lll::symmertric_lll_checker(&dep) );
    println!( "shearer's bound : {:?}", lll::shearers_bound_checker(&dep) );

    let dep = dep::DependencyGraph::from_sat_with_match(&sat, &mat);
    println!( "my new bound    : {:?}", lll::shearers_bound_checker(&dep) );


    println!( "satisfiablity   : {:?}", lll::satisfiability_checker(&sat) );

    println!(
        "MT  : {:?}",
        moser_tardos_algorithm::bench_algorithm::<
            moser_tardos_algorithm::MTsAlgorithmSimulator<
                random_space::InfiniteRandomSpace
            >
        >(&sat, 10000)
    );

    println!(
        "New : {:?}",
        moser_tardos_algorithm::bench_algorithm::<
            moser_tardos_algorithm::NewAlgorithmSimulator<
                random_space::InfiniteRandomSpace
            >
        >(&sat, 10000)
    );
}
