mod lib;

fn main() {
    let mut arr: [i32; 4] = [10, 0, 12 ,32];
    println!("Before myFunction: {:?}", arr);
    lib::question7(&mut arr);
    println!("After myFunctin:  {:?}\n", arr);
}
