use crate::sat::SAT;

pub struct Match {
    edge: Vec<(usize, usize)>
}

impl Match {
    fn from_random(n: usize) -> Self {
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

    fn from_sat_greedy(sat: SAT) -> Self {
        todo!()
    }
}