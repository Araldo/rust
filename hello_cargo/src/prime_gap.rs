#![allow(dead_code)]

struct PrimeSet {
    primes: Vec<u64>,
}

impl PrimeSet {
    fn new() -> Self {
        PrimeSet { primes: vec![2, 3] }
    }

    fn is_prime(&self, n: u64) -> bool {
        let max_p = (n as f64).sqrt() as u64;
        for p in self.primes.iter() {
            if n % p == 0 {
                return false;
            }
            if p > &max_p {
                break;
            }
        }
        true
    }

    fn find_next(&mut self) -> u64 {
        // add next prime to the primes set
        // print!("{:?}", self.primes);
        let mut n = *self.primes.last().unwrap();
        loop {
            n += 2;
            if self.is_prime(n) {
                self.primes.push(n);
                return n;
            }
        }
    }

    fn find(&mut self, n: u64) -> u64 {
        let mut m = *self.primes.last().unwrap();
        while m < n {
            m = self.find_next();
        }
        m
    }
}

fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    // Returns first two subsequent primes with gap g in between m and n.
    // Return None if such a set does not exist.
    let mut pset = PrimeSet::new();

    let mut first = pset.find(m);
    while first <= n {
        let second = pset.find(first + 2);
        if (second - first) as i32 == g {
            if second > n {
                return None;
            }
            return Some((first, second));
        }
        first = second;
    }
    None
}

#[allow(dead_code)]
fn testing(g: i32, m: u64, n: u64, result: Option<(u64, u64)>) {
    assert_eq!(gap(g, m, n), result);
}

#[test]
pub(crate) fn basics_gap() {
    testing(2, 100, 110, Some((101, 103)));
    testing(4, 100, 110, Some((103, 107)));
    testing(6, 100, 110, None);
    testing(8, 300, 400, Some((359, 367)));
}
