use std::fmt::Write;

#[test]
fn check_random_space_iterator() {
    use crate::random_space:: random_space_of_nbits;

    let bits : Vec<_> = random_space_of_nbits(10).collect();
    let str1 = format!( "{:?}", bits[460] );
    let str2 = format!( "{:010b}", 460 );
    let str2: String = str2.chars().rev().collect();
    assert_eq!( str1, str2 );
}

#[test]
fn check_moser_tardos_simulator() {
    use crate::sat;
    use crate::dep;
    use crate::moser_tardos_algorithm;
    use crate::random_space;

    let sat = [[-1, 2, 4], [1, 3, 5], [2, 4, 5], [-4, 5, 6]];
    let sat = sat::SAT::from_array(sat);
    let dep = dep::DependencyGraph::form_sat(&sat);

    let mut sand_box = 
        moser_tardos_algorithm::AlgorithmSimulator::new(
            sat.clone(), random_space::random_random_space()
        );
    
    assert!( match sand_box.run_until_terminal() {
        moser_tardos_algorithm::ExecuteResult::Terminal{ .. } => true,
        _ => false
    }, );

    let var = sand_box.get_varible();
    for c in &sat {
        assert!( !c.check_violated(var) );
    }
}