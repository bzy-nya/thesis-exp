use std::f64::NAN;

use crate::lll::PredictedResult;
use crate::moser_tardos_algorithm::BenchResult;
use crate::sat;
use crate::lll;
use crate::dep;
use crate::moser_tardos_algorithm;
use crate::r#match;
use crate::random_space;

pub struct DataSet {
    dataset_name: String,
    data_names: Vec<String>
}

pub struct DataSetIterator<'a> {
    inner: std::slice::Iter<'a, String>
}

impl<'a> Iterator for DataSetIterator<'a> {
    type Item = sat::SAT;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            None => { None }
            Some( str ) => {
                let path = format!("./dataset/{str}");
                Some( sat::SAT::from_file(&path) )
            }
        }
    }
}

impl<'a> IntoIterator for &'a DataSet {
    type Item = sat::SAT;
    type IntoIter = DataSetIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        DataSetIterator {
            inner: self.data_names.iter()
        }
    }
}

impl DataSet {
    fn size(&self) -> usize {
        self.data_names.len()
    }
}

pub fn load_dataset(name: &str) -> DataSet {
    let path = format!("./dataset/{name}.txt");
    let content = std::fs::read_to_string(path)
        .expect("Should have been able to read the file");

    DataSet {
        dataset_name: String::from(name),
        data_names: content.split("\n").map(|s| String::from(s)).collect()
    }
}

pub fn bench(dataset: &DataSet, turn: usize, data_filter: &str) {
    println!( "id, LLL, SHE, New, Con, BF, mt_mean, mt_sigma, z1, z2, z3, z4" );

    let mut skipped_cnt = 0;
    let mut failed_cnt = 0;
    let mut c1 = 0; let mut d1 = 0; let mut e1 = 0;
    let mut c2 = 0; let mut d2 = 0; let mut e2 = 0;
    let mut c3 = 0; let mut d3 = 0; let mut e3 = 0;
    let mut c4 = 0; let mut d4 = 0; let mut e4 = 0; 

    let sqn = (turn as f64).sqrt();
    let z99 = 2.33;

    for (id, sat) in dataset.into_iter().enumerate() {
        eprintln!( "bench on {} {}/{}", dataset.dataset_name, id + 1, dataset.size() );

        let mat = if sat.size() <= 10000 
            { r#match::Match::from_sat_greedy(&sat) }
        else 
            { r#match::Match::from_random(sat.size()) };
        
        let dep = dep::DependencyGraph::form_sat(&sat);

        let lll = lll::symmertric_lll_checker(&dep);
        let she = lll::shearers_bound_checker(&dep);

        let dep = dep::DependencyGraph::from_sat_with_match(&sat, &mat);
        let new = lll::shearers_bound_checker(&dep);

        let dep = dep::DependencyGraph::from_sat_with_match_conjecture(&sat, &mat);
        let con = lll::shearers_bound_checker(&dep);

        let bf = lll::satisfiability_checker(&sat);

        let skiped = match data_filter {
            "lll" => { lll == PredictedResult::Invalid },
            "she" => { she == PredictedResult::Invalid },
            "new" => { new == PredictedResult::Invalid },
            "con" => { con == PredictedResult::Invalid },
            "bf"  => { bf  == PredictedResult::Invalid },
            _ => {false}
        };

        if skiped {
            println!("{id}, skipped");
            skipped_cnt += 1;
            continue;
        }

        let mt = 
            moser_tardos_algorithm::bench_algorithm::<
                moser_tardos_algorithm::MTsAlgorithmSimulator<
                    random_space::InfiniteRandomSpace
                >
            >(&sat, turn);

        let pmt = 
            moser_tardos_algorithm::bench_algorithm::<
                moser_tardos_algorithm::NewAlgorithmSimulator<
                    random_space::InfiniteRandomSpace
                >
            >(&sat, turn);
        
        let (mean_x, sigma_x) = match mt {
            BenchResult::Failed => { 
                failed_cnt += 1;
                println!("failed"); continue; 
            }
            BenchResult::Success { mean, sigma } => { ( mean, sigma) }
        };

        let (mean_y, sigma_y) = match pmt {
            BenchResult::Failed => { 
                failed_cnt += 1;
                println!("failed"); continue; 
            }
            BenchResult::Success { mean, sigma } => { ( mean, sigma) }
        };

        let z1 = match new {
            PredictedResult::Invalid => { NAN },
            PredictedResult::UpperBound(p1) => {(mean_x - p1) /  (sigma_x / sqn) }
        };

        let z2 = match con {
            PredictedResult::Invalid => { NAN },
            PredictedResult::UpperBound(p2) => {(mean_x - p2) /  (sigma_x / sqn) }
        };

        let z3 = match bf {
            PredictedResult::Invalid => { NAN },
            PredictedResult::UpperBound(p3) => {(mean_x - p3) /  (sigma_x / sqn) }
        };

        let z4 = (mean_y - mean_x) / ( (sigma_x * sigma_x + sigma_y * sigma_y).sqrt() / sqn );

        if z1.is_nan() { d1 += 1; }
        else if z1 < z99 { c1 += 1; }
        else { e1 += 1; }

        if z2.is_nan() { d2 += 1; }
        else if z2 < z99 { c2 += 1; }
        else { e2 += 1; }

        if z3.is_nan() { d3 += 1; }
        else if z3 < z99 { c3 += 1; }
        else { e3 += 1; }

        if z4.is_nan() { d4 += 1; }
        else if z4 < z99 { c4 += 1; }
        else { e4 += 1; }

        println!("{id},{lll},{she},{new},{con},{bf},{mt},{z1:.3},{z2:.3},{z3:.3},{z4:.3}")
    }
    eprintln!("skipped: {skipped_cnt}");
    eprintln!("failed: {failed_cnt}");
    eprintln!("z1 : pass: {c1} fail: {e1} nan: {d1}");
    eprintln!("z2 : pass: {c2} fail: {e2} nan: {d2}");
    eprintln!("z3 : pass: {c3} fail: {e3} nan: {d3}");
    eprintln!("z4 : pass: {c4} fail: {e4} nan: {d4}");
}

pub fn enum_step(dataset: &DataSet, turn: usize, data_filter: &str) {
    let str1 = (0..=turn).map( |x| format!("MT{x}") ).collect::<Vec<String>>().join(", ");
    let str2 = (0..=turn).map( |x| format!("New{x}") ).collect::<Vec<String>>().join(", ");
    println!( "id, LLL, SHE, New, BF, {str1}, {str2}" );

    for (id, sat) in dataset.into_iter().enumerate() {
        eprintln!( "enum on {} {}/{}", dataset.dataset_name, id + 1, dataset.size() );

        let mat = r#match::Match::from_sat_greedy(&sat);
        let dep = dep::DependencyGraph::form_sat(&sat);

        let lll = lll::symmertric_lll_checker(&dep);
        let she = lll::shearers_bound_checker(&dep);

        let dep = dep::DependencyGraph::from_sat_with_match(&sat, &mat);
        let new = lll::shearers_bound_checker(&dep);

        let bf = lll::satisfiability_checker(&sat);

        let skiped = match data_filter {
            "lll" => { lll == PredictedResult::Invalid },
            "she" => { she == PredictedResult::Invalid },
            "new" => { new == PredictedResult::Invalid },
            _ => {false}
        };

        if skiped {
            println!("{id}, skiped");
            continue;
        }

        let mt = 
            moser_tardos_algorithm::enum_algorithm::<
                moser_tardos_algorithm::MTsAlgorithmSimulator<
                    random_space::LimitedRandomSpace
                >
            >(&sat, turn);

        let pmt = 
            moser_tardos_algorithm::enum_algorithm::<
                moser_tardos_algorithm::NewAlgorithmSimulator<
                    random_space::LimitedRandomSpace
                >
            >(&sat, turn);
        
        println!("{id},{lll},{she},{new},{bf},{mt},{pmt}")

    }
}

pub fn run(dataset: &DataSet) {
    let mut cnt = 0;
    for (id, sat) in dataset.into_iter().enumerate() {
        eprintln!( "run on {} {}/{}", dataset.dataset_name, id + 1, dataset.size() );

        let dep = dep::DependencyGraph::form_sat(&sat);

        match lll::symmertric_lll_checker(&dep) {
            PredictedResult::Invalid => { continue; }
            _ => {}
        }

        if moser_tardos_algorithm::run_algorithm::
            <moser_tardos_algorithm::MTsAlgorithmSimulator
                <random_space::InfiniteRandomSpace
            >>(&sat)
          { cnt += 1; }
    }

    eprintln!("{cnt}");
}