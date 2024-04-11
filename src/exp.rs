use crate::sat;
use crate::lll;
use crate::dep;
use crate::moser_tardos_algorithm;
use crate::r#match;
use crate::random_space;

pub struct DataSet {
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

pub fn load_dataset(name: &str) -> DataSet {
    let path = format!("./dataset/{name}.txt");
    let content = std::fs::read_to_string(path)
        .expect("Should have been able to read the file");

    DataSet {
        data_names: content.split("\n").map(|s| String::from(s)).collect()
    }
}

pub fn bench(dataset: &DataSet, turn: usize) {
    println!( "id, LLL, SHE, New, BF, MT mean, MT L995, New P-MT Mean, New P-MT L995" );

    for (id, sat) in dataset.into_iter().enumerate() {

        let mat = r#match::Match::from_sat_greedy(&sat);
        let dep = dep::DependencyGraph::form_sat(&sat);

        let lll = lll::symmertric_lll_checker(&dep);
        let she = lll::shearers_bound_checker(&dep);

        let dep = dep::DependencyGraph::from_sat_with_match(&sat, &mat);
        let new = lll::shearers_bound_checker(&dep);


        let bf = lll::satisfiability_checker(&sat);

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
        
        println!("{id},{lll},{she},{new},{bf},{mt},{pmt}")
    }
}