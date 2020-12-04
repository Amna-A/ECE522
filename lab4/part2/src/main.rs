
mod lib;

pub fn main() {
    print!("{:#?}",lib::main());

}
    //Q6: The output (iter)
    /*
        The average age of people older than 30 is 36.5
        - Overall, for each test with a vector size 1000, 10000, 100000, and 1000000
        - the parallel version was actually slower. My guess is that these tests are
        - small enough that the overhead for parallelization actually outweighs its benefits.
    */ 
//Q8: Report the benchmarking output. Consider comparing your program with the original
//code that just uses iter(). Does using par_iter make a difference?


//Q9:  Benchmark both the program with different vector sizes (1000, 10000, 100000, and 1000000) and provide your comments.

