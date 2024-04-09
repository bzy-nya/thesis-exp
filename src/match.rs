use core::slice;

use crate::sat::SAT;

pub struct Match {
    edge: Vec<(usize, usize)>
}

impl Match {
    pub fn from_random(n: usize) -> Self {
        use rand::prelude::*;

        let mut rng = rand::thread_rng();
        let mut nums: Vec<usize> = (0..n).collect();
        nums.shuffle(&mut rng);
        
        let mut edge = Vec::new();
        for i in 0..n/2 {
            edge.push((nums[i * 2], nums[i * 2 + 1]));
        }
        
        Self {
            edge
        }
    }

    pub fn from_sat_greedy(sat: &SAT) -> Self {
        let mut edge = Vec::new();

        let n = sat.size();
        let mut choosed = crate::new_vector(n, false);

        for u in 0..n { if !choosed[u] {
            let mut candidate = None;
            let mut mx = 0.0;

            for v in (u + 1..n).filter( |&x| !choosed[x] ) {
                let pairwise_pr = sat.pr_land( &[u,v].into() );
                if mx < pairwise_pr {
                    mx = pairwise_pr;
                    candidate = Some(v);
                }
            }

            match candidate {
                Some(v) => {
                    edge.push((u, v));
                    choosed[u] = true;
                    choosed[v] = true;
                }
                _ => {}
            }
        }}

        Self{ edge }
    }
}

impl IntoIterator for Match {
    type Item = (usize, usize);
    type IntoIter = std::vec::IntoIter<(usize, usize)>;

    fn into_iter(self) -> Self::IntoIter {
        self.edge.into_iter()
    }
}

impl<'a> IntoIterator for &'a Match {
    type Item = &'a (usize, usize);
    type IntoIter = slice::Iter<'a, (usize, usize)>;

    fn into_iter(self) -> Self::IntoIter {
        self.edge.iter()
    }
}