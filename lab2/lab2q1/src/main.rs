
fn main() {
    let mut groups = [[""; 4]; 6];
    groups[0]=["Bob", "Carol", "Eric", "Matt"];
    groups[1]=["Jim", "Lucy", "Terry", "Brenda"];
    groups[2]=["Susan", "Brad", "Jim", "Matt"];
    groups[3]=["Sue", "Wendy", "Sam", "Brad"];
    groups[4]=["Kate", "Jack", "James", "Sydney"];
    groups[5]=["Mary", "John", "Ricky", "Wendy"];
    
    searchMember(groups , "Brad");
}
#[allow(non_snake_case)]
pub fn searchMember(arr: [[&str;4];6], name: &str){

    let mut found:Vec<&str> = Vec::new();
    let mut group_num:Vec<usize> = Vec::new();
    let mut group_leader:Vec<String> = Vec::new();

    for (i,j) in arr.iter().enumerate(){
        if j.contains(&name){
            group_num.push(i);
            found.push("yes");
        }
        else{
            found.push("no");
        } 
        if arr[i][0].contains(&name){
            group_leader.push((name).to_string());
        }
    }
    //....................printing......................................
    if found.contains(&"yes"){
        println!("'{}' exists on the list",name);
        println!("'{}' group number(s): {:?}",name,group_num);
        if group_leader.contains(&name.to_string()){
            println!("'{}' is a group leader",name);
        }//group leader is the first member of the list
        else if group_leader.contains(&name.to_string()) == false{
            println!("'{}' is not a group leader",name);
        } 
    }
    else if found.contains(&"no"){
        println!("'{}' is not part of the list",name);
    }
    
}
    
