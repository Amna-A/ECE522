fn main() {
    list_of_primes(1000); 
}
pub fn is_prime(num: i128) -> bool {
    if num <=1 {
        return false}
        for i in 2..num {
            if num % i == 0{
                return false;
            }
    }true
}
pub fn list_of_primes(limit: i128) {
    
    let mut primes:Vec<i128> = vec![];
    for p in 2..limit{
        if is_prime(p){
            primes.push(p);
        }
    }
    let mut maxsum = 0;
    let mut maxlist = Vec::new();
    for i in primes.clone() {
        let mut sum = 0;
        let mut list = Vec::new();
        for num in i..(primes[primes.len()-1] + 1)
        {
            if is_prime(num) {
                sum+=num;
                if sum > 1000 {
                    break;
                }
                list.push(num);
            
                if list.len() > maxlist.len() && is_prime(sum) {
                    maxlist = list.clone();
                    maxsum = sum;
                }
            }
        }
    }
    println!("The longest sum of primes under '{}' contains '{:?}' terms",limit,maxlist.len());
    println!("and is equal to: {:?}",maxsum);
    println!("The list of primes bellow {} is: {:?}",limit,maxlist);
}
