
fn main() {
    //create a struct value
    let mut value = TreeNode {
        data: "5",
        left_child: None,
        right_child: None
    };
    //start filling the tree
    value.insert_node("2");
    value.insert_node("3");
    value.insert_node("7");
    value.insert_node("6");

    //print the tree
    println!("{:#?}",value);
}

#[derive(Debug)]
struct TreeNode<'a> {
    data: &'a str,
    left_child: Option<Box<TreeNode<'a>>>,
    right_child: Option<Box<TreeNode<'a>>>,
}
impl<'a> TreeNode<'a>{
    pub fn insert_node(&mut self, new_data: &'a str) {
        if self.data == new_data {
        return
        }
        let new_node = if new_data < self.data { &mut self.left_child } else {
        &mut self.right_child };
        match new_node{
            &mut None => {
                let new = TreeNode {data: new_data, left_child: None, right_child: None };
                let boxed = Some(Box::new(new));
                *new_node = boxed;
            }
            &mut Some(ref mut child_node) => child_node.insert_node(new_data),
        }
    }
}
