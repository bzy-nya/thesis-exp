use core::slice;
use std::collections::BTreeSet;

#[derive(Clone)]
pub struct Clause {
    literal : Vec<isize>
}

#[derive(Clone)]
pub struct SAT {
    // k-CNF SAT
    n: usize,              // number of clause
    m: usize,              // number of variable
    k: usize,              // number of literal in one clause
    clause : Vec<Clause>
}

impl Clause {
    pub fn new(k: usize) -> Self {
        Self { literal: crate::new_vector(k, 0) }
    } 

    pub fn from_vec(v: Vec<isize>) -> Self {
        Self { literal: v.into_iter().filter(|&x| x != 0).collect() }
    }

    pub fn from_array<const L: usize>(array: [isize; L]) -> Self {
        Self { literal: Vec::from(array) }
    }

    pub fn check_violated(&self, var: &Vec<bool>) -> bool {
        self.literal.iter().map(
            |liter| {
                let pos = (liter.abs() - 1) as usize;
                if *liter < 0 { !var[pos] } else { var[pos] }
            }
        ).reduce(|x, y| x & y ).unwrap()
    }

    pub fn size(&self) -> usize {
        self.literal.len()
    }

    pub fn collect_varible(&self) -> Vec<usize> {
        self.literal.iter().map(|x| x.abs() as usize).collect()
    }
}

impl<'a> IntoIterator for &'a Clause {
    type Item = &'a isize;
    type IntoIter = slice::Iter<'a, isize>;

    fn into_iter(self) -> Self::IntoIter {
        self.literal.iter()
    }
}

impl std::fmt::Debug for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.literal)
    }
}

impl SAT {
    pub fn new(n: usize, m: usize, k: usize) -> Self {
        SAT {
            n, m, k, 
            clause: crate::new_vector(n, Clause::new(k))
        }
    }

    pub fn from_file(path: &str) -> Self {
        let content = std::fs::read_to_string(path)
            .expect("Should have been able to read the file");

        let vec = content.split("\n").skip(1)
            .filter(|&line| {line != ""} )
            .map( |line| {
                    line.split(" ")
                        .map( |x| x.parse::<isize>().unwrap() )
                        .filter( |&x| x != 0 )
                        .collect()
                } )
            .collect();

        Self::from_vec(vec)
    }

    pub fn from_vec(vec: Vec<Vec<isize>>) -> Self {
        let mut k = vec[0].len();
        for v in &vec 
          { if v.len() != k {k = 0;} }
        
        if vec[0].ends_with(&[0]) && k != 0
          { k -= 1 }

        let mut set = std::collections::BTreeSet::new();
        
        for v in &vec {
            for l in v {
                if *l != 0 { set.insert(l.abs()); }
            }
        }
        
        Self {
            n: vec.len(),
            m: set.len().max( *set.iter().max().unwrap() as usize ),
            k,
            clause: vec.into_iter().map( Clause::from_vec ).collect()
        }
    }

    pub fn from_array<const K: usize, const N : usize>(array: [[isize; K]; N]) -> Self {
        Self::from_vec( array.into_iter().map( Vec::from ).collect() )
    }

    pub fn check_violated(&self, var: &Vec<bool>) -> Vec<bool> {
        self.clause.iter().map( |c| { Clause::check_violated(c, var) } ).collect()
    }

    pub fn pr_land( &self, clauses_id: &Vec<usize> ) -> f64 {
        if self.size() >= 1000 
            { return 0.0; }
        let mut set = BTreeSet::new();
        
        for &c in clauses_id {
            for &l in self.get_clause(c) {
                if set.contains(&(-l)) { return 0.0 };
                set.insert(l);
            }
        }
        
        return 1.0 / (1 << set.len()) as f64
    }

    pub fn get_clause(&self, id: usize) -> &Clause {
        &self.clause[id]
    }

    pub fn size(&self) -> usize { self.n }

    pub fn variable_count(&self) -> usize { self.m }

    pub fn clause_size(&self) -> usize { self.k }
}

impl std::fmt::Debug for SAT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.clause )
    }
}

impl<'a> IntoIterator for &'a SAT {
    type Item = &'a Clause;
    type IntoIter = slice::Iter<'a,Clause>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.clause.iter()
    }
}