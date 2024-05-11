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
    use crate::moser_tardos_algorithm;
    use crate::moser_tardos_algorithm::AlgorithmSimulator;
    use crate::random_space;

    let sat = [[-1, 2, 4], [1, 3, 5], [2, 4, 5], [-4, 5, 6]];
    let sat = sat::SAT::from_array(sat);

    let mut sand_box = 
        moser_tardos_algorithm::MTsAlgorithmSimulator::new
            (sat.clone(), random_space::random_random_space());

    
    assert!( match sand_box.run_until_terminal() {
        moser_tardos_algorithm::ExecuteResult::Terminal{ .. } => true,
        _ => false
    }, );

    let var = sand_box.get_varible();
    for c in &sat {
        assert!( !c.check_violated(var) );
    }
}

#[test]
fn enuming_test() {
    use crate::sat;
    use crate::moser_tardos_algorithm;
    use crate::random_space;

    let sat = [[1], [2], [3], [4], [5]];
    let sat = sat::SAT::from_array(sat);
    
    let result = moser_tardos_algorithm::enum_algorithm::<
        moser_tardos_algorithm::MTsAlgorithmSimulator<
            random_space::LimitedRandomSpace
        >
    >(&sat, 2);

    match result {
        moser_tardos_algorithm::EnumResult::Failed => { assert!(false); }
        moser_tardos_algorithm::EnumResult::Success { p } => {
            let str: String = format!("{}", p[0]);
            assert_eq!( str, "0.96875" );
        }
    }
}

#[test]
fn lll_test() {
    use crate::lll;
    use crate::sat::SAT;
    use crate::dep::DependencyGraph;

    let sat = [[1, 2], [-2, -3], [3, 4]];
    let sat = SAT::from_array(sat);
    let dep = DependencyGraph::form_sat(&sat);

    println!( "{}", lll::satisfiability_checker(&sat) );
    println!( "{}", lll::shearers_bound_checker(&dep) );
}

#[test]
fn non_cnf_test() {
    use crate::sat::SAT;
    use crate::dep::DependencyGraph;

    let sat = [[1, 0], [-2, 0], [3, 0]];
    let sat = SAT::from_array(sat);
    println!("{:?}", sat.clause_size());

    let sat = vec![ vec![1, 2, 0], vec![1, 0], vec![3, 4, 5, 0] ];
    let sat = SAT::from_vec(sat);

    println!("{:?}", sat.variable_count());

    let dep = DependencyGraph::form_sat(&sat);

    println!("{}", dep.max_p);
}