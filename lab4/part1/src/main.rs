//........C functions...............
extern {
    pub fn abs(i: i32) -> i32;
}
pub fn compute_manhattan_distance_c(p1: &Point, p2: &Point) -> i32 {
    unsafe {
        let a_abs = abs(p2.x as i32 - p1.x as i32);
        let b_abs = abs(p2.y as i32 - p1.y as i32);
        a_abs + b_abs
    }
}
//.........main....................
fn main() {

    let x1:i32 = -2;
    print!("x: {:?}\n", x1);

    unsafe {
        println!("abs(x): {:?}",abs(x1));
    }
    println!("-----------------CLI----------------------");
    //point 1
    println!("Hello, Please enter x for point 1: ");
    use std::io;
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

    let trimmed_x1 = input_text.trim();
    match trimmed_x1.parse::<i8>() {
        Ok(x) => println!("The value of x: {}", x),
        Err(..) => println!("this was not an integer: {}", trimmed_x1),
    };
    println!("Then enter y for point 1: ");

    let mut input_text_2 = String::new();
    io::stdin().read_line(&mut input_text_2).expect("failed to read from stdin");

    let trimmed_y1 = input_text_2.trim();
    match trimmed_y1.parse::<i8>() {
        Ok(y) => println!("The value of y: {}", y),
        Err(..) => println!("this was not an integer: {}", trimmed_y1),
    };
    //point 2
    println!("Now enter x for point 2: ");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

    let trimmed_x2 = input_text.trim();
    match trimmed_x2.parse::<i8>() {
        Ok(x) => println!("The value of x: {}", x),
        Err(..) => println!("this was not an integer: {}", trimmed_x2),
    };
    println!("Then enter y for point 2: ");

    let mut input_text_2 = String::new();
    io::stdin().read_line(&mut input_text_2).expect("failed to read from stdin");

    let trimmed_y2 = input_text_2.trim();
    match trimmed_y2.parse::<i8>() {
        Ok(y) => println!("The value of y: {}", y),
        Err(..) => println!("this was not an integer: {}", trimmed_y2),
    };
    use std::str::FromStr;

    let x1_value: i8 = FromStr::from_str(trimmed_x1).unwrap();
    let y1_value: i8 = FromStr::from_str(trimmed_y1).unwrap();

    let x2_value: i8 = FromStr::from_str(trimmed_x2).unwrap();
    let y2_value: i8 = FromStr::from_str(trimmed_y2).unwrap();

    let p1 = Point{x: x1_value, y: y1_value,};
    let p2 = Point{x: x2_value, y: y2_value,};

    println!("Which Distance you want to claculate? Enter the number of it");
    println!("1- Euclidean distance\n2- Manhattan distance\n3- Chebyshev Distance");

    let mut input_text_3 = String::new();
    io::stdin().read_line(&mut input_text_3).expect("failed to read from stdin");

    let trimmed = input_text_3.trim();
    match trimmed.parse::<i8>() {
        Ok(c) => println!("Your Choice: {}", c),
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    let choice: i8 = FromStr::from_str(trimmed).unwrap();
    if choice == 1{
        println!("The Euclidean distance between your points is: {}",compute_euclidean_distance(&p1, &p2));
    }else if choice == 2{
        println!("The Manhattan distance between your points is: {}",compute_manhattan_distance(&p1, &p2));
    }else if choice == 3{
        println!("The Chebyshev Distance between your points is: {}",compute_chebyshev_distance(&p1, &p2));
    }else{println!("Valid Input");}  
}
//....................................
#[repr(C)]
pub struct Point {
    x: i8,
    y: i8,
}
//Q1 Euclidean distance
pub fn compute_euclidean_distance(p1: &Point, p2: &Point) -> f64{
    let a_sqrt = (p2.x as f64 - p1.x as f64).powf(2.0);
    let b_sqrt = (p2.y as f64 - p1.y as f64).powf(2.0);
    (a_sqrt+b_sqrt).sqrt()
}
pub fn compute_manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    let a_abs = (p2.x as i32 - p1.x as i32).abs();
    let b_abs = (p2.y as i32 - p1.y as i32).abs();
    a_abs + b_abs
}
   
//Q2 Test the Euclidean distance function
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_euclidean_distance() {
            let p1 = Point{x: 5, y: 4,};
            let p2 = Point{x: 1, y: 1,};
            assert_eq!(compute_euclidean_distance(&p1, &p2), 5.0);
    }
    #[test]
    fn test_chebyshev_distance(){
        let p1 = Point{x: 1, y: 1,};
        let p2 = Point{x: 5, y: 4,};
        assert_eq!(compute_chebyshev_distance(&p1, &p2), 4); 
    }
    #[test]
    fn test_chebyshev_distance_c(){
        let p1 = Point{x: 1, y: 1,};
        let p2 = Point{x: 5, y: 4,};
        assert_eq!(compute_chebyshev_distance_c(&p1, &p2), 4); 
    }
    #[test]
    fn test_manhattan_distance(){
        let p1 = Point{x: 2, y: 3,};
        let p2 = Point{x: 3, y: 5,};
        assert_eq!(compute_manhattan_distance(&p1, &p2), 3); 
    }
    #[test]
    fn test_manhattan_distance_c(){
        let p1 = Point{x: 2, y: 3,};
        let p2 = Point{x: 3, y: 5,};
        assert_eq!(compute_manhattan_distance_c(&p1, &p2), 3); 
    }
} 
//Q3 compute the ChebyshevDistance
use core::cmp::max;
pub fn compute_chebyshev_distance(p1: &Point, p2: &Point) -> i32{
    let a_abs = (p2.x as i32 - p1.x as i32).abs();
    let b_abs = (p2.y as i32 - p1.y as i32).abs();
    max(a_abs, b_abs)   
}
//Q4 apply FFI to write chebyshev_distance_c
pub fn compute_chebyshev_distance_c(p1: &Point, p2: &Point) -> i32{
    unsafe
    {
        let a_abs = abs(p2.x as i32 - p1.x as i32);
        let b_abs = abs(p2.y as i32 - p1.y as i32);
        max(a_abs, b_abs)
    }  
}