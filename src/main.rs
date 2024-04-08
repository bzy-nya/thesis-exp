mod utility;
use utility::*;

mod sat;
mod dep;
mod lll;
mod r#match;
mod random_space;
mod moser_tardos_algorithm;

use moser_tardos_algorithm::AlgorithmSimulator;

#[cfg(test)]
mod tests;

fn main() {
    let sat = [[-1, 2, 4], [1, 3, 5], [2, 4, 5], [-3, 5, 6]];
    let sat = [[1,2],[1,-2],[-1, 2]];
    let sat = sat::SAT::from_array(sat);
    let dep = dep::DependencyGraph::form_sat(&sat);

    println!( "lll predict    : {:?}", lll::symmertric_lll_checker(&dep) );
    //println!( "sheare's bound : {:?}", lll::shearers_bound_checker(&dep) );
    println!( "satisfiablity  : {:?}", lll::satisfiability_checker(&sat) );

    let mut sand_box1 = 
        moser_tardos_algorithm::MTsAlgorithmSimulator::new(
            sat.clone(), random_space::random_random_space()
        );

    let mut sand_box2 = 
        moser_tardos_algorithm::NewAlgorithmSimulator::new(
            sat.clone(), random_space::random_random_space()
        );
    
    sand_box1.run_init();
    sand_box2.run_init();

    for x in sand_box1 {
        println!("{:?}", x);
    }

    for x in sand_box2 {
        println!("{:?}", x);
    }
}
