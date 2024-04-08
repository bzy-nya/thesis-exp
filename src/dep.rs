use crate::sat::SAT;
use crate::r#match::Match;
use std::collections::BTreeSet;

pub struct DependencyGraph {
    pub n: usize,    // number of vertices
    pub m: usize,    // number if edges
    edge:  Vec<(usize, usize)>,
    gamma: Vec<Vec<usize>>,
    pub p: Vec<f64>,
    pub max_d: usize,
    pub max_p: f64
}

impl DependencyGraph {
    pub fn form_sat(sat: &SAT) -> DependencyGraph {
        let n = sat.size();
        let mut edge  = Vec::new();
        let mut gamma = Vec::new();
        let p = crate::new_vector(n, 1.0 / (1 << sat.clause_size()) as f64 );
        let mut max_d = 0;
        let max_p = p[0];

        let mut cache = crate::new_vector(sat.variable_count(), BTreeSet::new());

        for (id, c) in sat.into_iter().enumerate() {
            let mut neighbor = BTreeSet::new();

            for v in c.collect_varible() {
                for pre in &cache[v - 1] {
                    neighbor.insert(pre);
                }
            }

            max_d = max_d.max( neighbor.len() );
            
            for &&pre in &neighbor {
                edge.push((pre, id));
                let qwq : &mut Vec<usize> = &mut gamma[pre];
                qwq.push(id);
            }

            gamma.push(Vec::from_iter(neighbor.into_iter().map(|x| *x)));

            
            for v in c.collect_varible() {
                cache[v - 1].insert(id);
            }
        }

        Self {
            n, m: edge.len(),
            edge, gamma, p, max_d, max_p
        }
    }

    pub fn from_sat_with_match(sat: &SAT, mat: &Match) -> DependencyGraph {
        let mut dep = Self::form_sat(sat);
        todo!();
        dep
    }

    pub fn get_gamma(&self, index: usize) -> &Vec<usize> {
        &self.gamma[index]
    }

    pub fn get_gamma_plus(&self, id: usize) -> Vec<usize> {
        let mut ret = self.get_gamma(id).clone();
        ret.push(id);
        ret
    }
}

impl Clone for DependencyGraph {
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            m: self.m,
            edge: self.edge.clone(),
            gamma: self.gamma.clone(),
            p: self.p.clone(),
            max_d: self.max_d,
            max_p: self.max_p
        }
    }
}