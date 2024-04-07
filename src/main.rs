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
    let dep = dep::DependencyGraph::form_sat(&sat);

    println!( "lll predict    : {:?}", lll::symmertric_lll_checker(&dep) );
    //println!( "sheare's bound : {:?}", lll::shearers_bound_checker(&dep) );

    for x in 0..dep.n {
        println!( "{:?}", dep.get_dependency(x) );
    }

    let mut sand_box = 
        moser_tardos_algorithm::AlgorithmSimulator::new(
            sat, random_space::random_random_space()
        );
    
    sand_box.run_init();
    
    for x in sand_box {
        println!("{:?}", x);
    }
}
