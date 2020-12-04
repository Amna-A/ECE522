

fn main() {
    println!("{}",function(40));
}

use rand::prelude::Rng;
extern crate ramp;
use ramp::Int;
extern crate primal;
use primal::is_prime;

pub fn function(n: u64) -> Int {
    let mut rng = rand::thread_rng();
    loop {
        let mut candidate:Int= Int::from(rng.gen_range(0, n));
        candidate.set_bit(0, true);
        let i = u64::from(&candidate);
        if is_prime(i) == true{
            return candidate;
        }
    }
}




