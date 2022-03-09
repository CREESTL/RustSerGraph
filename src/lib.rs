#![allow(dead_code)]

// Export structs
pub use node::Node;
pub use tree::Tree;
pub use iter::GraphIter;


// Module of a graph node
mod node {
    // Alias for index in arena
    pub type Index = usize;

    // Struct of a graph node
    pub struct Node<T> {
        // Node has a value and other nodes connected to it
        pub value: T,
        pub left: Option<Index>,
        pub right: Option<Index>
    }

    impl<T> Node<T> {
        // Constructor for a new node
        pub fn new(value: T, left: Option<Index>, right: Option<Index>) -> Self{
            Node{value, left, right}
        }
    }
}


// Module of a graph
mod tree {
    use super::node::{Node, Index};
    use super::iter::GraphIter;

    // Struct of a graph
    pub struct Tree<T> {
        // Graph has a root and an arena
        // Arena is a vector holding nodes of a graph. Allows for random access without nested borrowing
        arena: Vec<Option<Node<T>>>,
        // A root is a member of arena so it's represented as an index
        root: Option<Index>,
    }

    impl<T> Tree<T>{
        // Constructor of a graph
        // At first, tree has no root. It must be set with set_root()
        pub fn new() -> Self {
            Tree{arena: Vec::new(), root: None}
        }

        // Function sets the root of a graph
        pub fn set_root(&mut self, root: Option<Index>) {
            self.root = root;
        }

        // Function adds a node to the graph
        // The index of that node in arena is returned
        pub fn add_node(&mut self, node: Node<T>) -> Index{
            let index = self.arena.len();
            self.arena.push(Some(node));
            return index
        }

        // Function removes a node from the graph
        pub fn remove_node(&mut self, index: Index) -> Option<Node<T>> {
            if let Some(node) = self.arena.get_mut(index) { 
                node.take()
            } else {
                None
            }
        }

        // Function gets the node from the graph (borrows it)
        pub fn get_node(&self, index: Index) -> Option<&Node<T>> {
            if let Some(node) = self.arena.get(index) {
                node.as_ref()
            } else {
                None
            }
        }

        // Function gets a mutable node from the graph (mutably borrows it)
        pub fn get_node_mut(&mut self, index: Index) -> Option<&mut Node<T>> {
            if let Some(node) = self.arena.get_mut(index) {
                node.as_mut()
            } else {
                None
            }
        }


        // Function returns a custom iterator over the graph
        pub fn iter(&mut self) -> GraphIter {
            GraphIter::new(self.root)
        }



    }
}


// Module of a custom iterator
// Built-in Iterator trait doesn't fit current task. It can't mutably borrow an element of a vector if
// the vector has been borrowed already
mod iter{
    use super::node::Index;
    use super::tree::Tree;

    pub struct GraphIter{
        // Iterator holds all data in the stack
        // This is an array of indexes of elements of the graph
        stack: Vec<Index>
    }

    impl GraphIter{
        // Constructor of the iterator 
        pub fn new(root: Option<Index>) -> Self {
            // If there is a root - stack starts with it
            if let Some(index) = root {
                GraphIter {
                    stack: vec![index]
                }
            // If there is no root - stack is empty
            } else {
                GraphIter {
                    stack: vec![]
                }
            }
        }

        // Function returns the next item from the iterator
        // This function implements a Visitor Pattern. It only borrows a graph when it's beeing called
        // Between the calls the graph can be modified in any way. A graph to borrow is passed as the second parameter
        pub fn next<T>(&mut self, tree: &Tree<T>) -> Option<Index> {
            // Get the next index from the stack
            while let Some(node_index) = self.stack.pop(){
                // Get the node with that index from the graph (from arena)
                if let Some(node) = tree.get_node(node_index) {
                    // Add it's neighbours to the stack
                    if let Some(right) = node.right {
                        self.stack.push(right);
                    }
                    if let Some(left) = node.left {
                        self.stack.push(left);
                    }

                    return Some(node_index)
                }
            }

            return None
        }
    }
}

