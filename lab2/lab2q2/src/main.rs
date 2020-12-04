/*
    - The code doe not run.
    - Rust needs to know at compile time how much space a type takes up. 
    - Rust stores the data inside a struct directly inline inside the memory of the struct value.
    - The TreeNode here is recrusively defined so it's an (inductive data type) ,
      meaning it's a data type for values that may contain other data types.
    - So the size of tTreeNode will be the size of it + size of all of it's elements.
    - Since we don't know yet the data on the children (right side),
      the size of the struct will have to be infinite.
    - The solution is to make it of a fixed size by boxing the right&left children since they r option enum
      so we still have the recrusive type.
    - the &str is a reference and we can not box it so we give it a lifetime 

*/
fn main() {
  
}
#[derive(Debug)]
struct TreeNode<'a> {
    data: &'a str,
    left_child: Box<Option<TreeNode<'a>>>,
    right_child: Box<Option<TreeNode<'a>>>,
}

