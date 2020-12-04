mod lib;
fn main() {
    let mut arr: [f32; 17] = [8.7, 8.5, 4.3, 2.9, 0.8, 1.5, 1.6, 2.4, 1.1,5.1, 4.8, 4.5, 6.1, 3.1, 7.9, 8.1, 2.0];
    println!("Before myFunction: {:?}", arr);
    lib::question2(&mut arr);
    println!("After myFunctin:  {:?}\n", arr);
}

