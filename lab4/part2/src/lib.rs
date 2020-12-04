use rayon::prelude::*;

pub struct Person {
    pub age: u32,
}
pub fn main() {
    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];
    println!("Output..Case1: non-parallel:");
    println!("The average age of people older than 30 is {}",average_age(&v));

    println!("Output..Case2: parallel):");
    println!("The average age of people older than 30 is {}",average_age_parallel(&v));
}

pub fn average_age(v: &Vec<Person>) -> f32 {
    let num_over_30 = v.iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30: u32 = v.iter().map(|x| x.age).filter(|&x| x > 30).sum();
    sum_over_30 as f32 / num_over_30
}
//Q7: Alter the previous program (Question 6) to use par_iter instead of iter.
pub fn average_age_parallel(v: &Vec<Person>) -> f32 {
    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30: u32 = v.par_iter().map(|x| x.age).filter(|&x| x > 30).sum();
    sum_over_30 as f32 / num_over_30
}