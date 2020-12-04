use std::cmp::Ordering;

fn main() {
    let mut value = Tree::new();
    value.insert_node(3);
    value.insert_node(2);
    value.insert_node(1);
    value.insert_node(5);
    value.insert_node(4);
    //value.insert_node("A");
    println!("{:#?}",value);
      
}
#[derive(Debug)]
pub enum Tree<T: Ord> {
    Node {
    data: T,
        left_child: Box<Tree<T>>,
        right_child: Box<Tree<T>>,
    },
    Empty,
}

impl<T: Ord> Tree<T> {

    pub fn new() -> Self {
       Tree::Empty
    }
    pub fn insert_node(&mut self, new_data: T){
        match self {
            //handle node with values (fill a node)
            &mut Tree::Node{ref data, ref mut left_child,ref mut right_child,} => {
                //compare the new data
                match new_data.cmp(data){
                    Ordering::Less => left_child.insert_node(new_data),
                    Ordering::Greater => right_child.insert_node(new_data),
                    _ => return
                }
            },
            //handle empty tree (fill it  up)
            &mut Tree::Empty => {
                *self = Tree::Node{
                    data:new_data,
                    left_child: Box::new(Tree::Empty),
                    right_child: Box::new(Tree::Empty)
                };
            }
        }
    }
}

/*
    changes:
    --------
        - add new function to handle an empty tree (creating  empty tree to not use none inside children)
        - instead of initiating the children with none use the Empty

    Purpose of Empty:
    -----------------
        - to tell the programe explicitly if there is a value or not
        - during runtime, the program might try to use null in case where an object is required. 
        this cause a null pointer exception. so using Empty in the enum help us solve this expection issue.
    
    Struct vs Enum:
    ---------------
        - the enum_based solution is better because filling the binary tree depends on checking 
        if a tree is empty or full. for binary trees it's an OR operation, so it makes since for this application to use an enum.
        while the struct is basically an AND operation wich is not fit very much for a binary tree. 
        each node at a level can have one state at a time which fits an enum structure more than a struct.

 */
