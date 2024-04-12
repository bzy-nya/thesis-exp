use rand::prelude::*;

pub trait RandomSpace : Iterator<Item = bool> {
    fn fetch_random_bit(&mut self) -> Option<bool>
      { self.next() }
    fn size(&self) -> usize;
}

pub struct InfiniteRandomSpace {
    rng: StdRng
}

impl InfiniteRandomSpace {
    pub fn new(seed: usize) -> Self {
        Self { 
            rng: StdRng::seed_from_u64(seed as u64)
        }
    }
}

impl Iterator for InfiniteRandomSpace {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        Some( self.rng.next_u32() & 1 == 1 )
    }
}

impl RandomSpace for InfiniteRandomSpace {
    fn size(&self) -> usize {
        2147483647
    }
}

pub struct LimitedRandomSpace {
    pointer: usize,
    seed: usize,
    limit: usize,
}

impl LimitedRandomSpace {
    pub fn new(seed: usize, limit: usize) -> Self {
        Self {
            pointer: 0, seed, limit
        }
    }
}

impl Iterator for LimitedRandomSpace {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pointer == self.limit
            { None }
        else {
            self.pointer += 1;
            Some((self.seed >> (self.pointer - 1)) & 1 == 1)
        }
    }
}

impl RandomSpace for LimitedRandomSpace {
    fn size(&self) -> usize {
        self.limit
    }
}

impl std::fmt::Debug for LimitedRandomSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str : String = (0..self.limit).map( |i| if (self.seed >> i) & 1 == 1 {"1"} else {"0"}).collect();
        write!(f, "{}", str)
    }
}

pub struct RandomSpaceIterator {
    bits: usize, 
    now: usize,
}

impl Iterator for RandomSpaceIterator {
    type Item = LimitedRandomSpace;
    fn next(&mut self) -> Option<Self::Item> {
        if self.now == 1 << self.bits
            { None }
        else {
            self.now += 1;
            Some(LimitedRandomSpace::new(self.now - 1, self.bits))
        }
    }
}

pub fn random_space_of_nbits(n: usize) -> RandomSpaceIterator {
    RandomSpaceIterator {
        bits: n,
        now: 0
    }
}

pub fn random_random_space() -> InfiniteRandomSpace {
    let mut rng = rand::thread_rng();
    InfiniteRandomSpace::new(rng.next_u64() as usize)
}