//question:2 sorting function in rust

pub fn question2(a: &mut [f32]){
    for i in 0..a.len(){
	    let mut min = i;
	    for j in (i + 1)..a.len() {
	        if a[j] < a[min] {
	            min = j;
	        }
	    }
	    a.swap(min, i);
	}
}
//Question 4: Does the optimization flag -O make a difference?
//yes, The compiler produced faster, smaller code in less compilation time