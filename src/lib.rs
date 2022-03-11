#![allow(dead_code)]

// Export structs
pub use node::Node;
pub use tree::Tree;
pub use iter::GraphIter;


// Module of a graph node
mod node {

    use std::fmt::Display;

    // Alias ti separate index of a node and index of a node in the arena vector
    type Number = usize;

    // Struct of a graph node]
    pub struct Node<T> {
        // Node has an index, a value and may have other nodes connected to it
        // Index is equal to node's position in arena of a graph.
        pub index: Option<usize>,
        pub value: T,
        // Connected nodes are accessed through their arena indexes
        // Deleted nodes are replaced with None. Length of vector stays the same
        pub connected: Vec<Option<usize>>
    }


    impl<T: Display> Node<T> {
        // Constructor for a new node
        pub fn new(value: T, connected: Vec<Option<usize>>) -> Self {
            // By default index of a node is None
            // Changes after a node is added to the arena
            Node{index: None, value, connected}
        }

        pub fn print_node(&self){
            println!("Current node:\n\tindex: {:?}\n\tvalue:{}\n\tChild nodes are: {:?}", self.index.unwrap(), self.value, self.connected);
        }

    }


}

// Module of a graph
mod tree {
    use super::node::Node;
    use super::iter::GraphIter;
    use std::fmt::Display;


    // Struct of a graph
    pub struct Tree<T> {
        // Graph has a root and an arena
        // Arena is a vector holding nodes of a graph. Allows for random access without nested borrowing
        // Access to each node from arena is through it's index.
        pub arena: Vec<Option<Node<T>>>,
        // Root is one of the nodes in arena. Access through index as well.
        pub root: Option<usize>,
    }

    impl<T: Display> Tree<T>{
        // Constructor of a graph
        // At first, tree has no root. It must be set with set_root()
        pub fn new() -> Self {
            Tree{arena: Vec::new(), root: None}
        }

        // Function adds a node to the graph
        pub fn add_node(&mut self, mut node: Node<T>) -> usize {            
            // Node's index is set equal to the current length of arena
            // Node's index does NOT indicate nodes order of proccessing.
            // Node with smaller index can be reached before the one with a bigger index
            // It is just an order of storing nodes in arena
            node.index = Some(self.arena.len());
            let arena_index = self.arena.len();
            self.arena.push(Some(node));
            // The index of a node is returned
            // Access to the node for any other operation is through this INDEX 
            // and not through the node's value!
            arena_index
        }


        // Function makes a node with a given arena index a root of a graph
        pub fn set_root(&mut self, root: Option<usize>) {
            self.root = root;
        }

        // Function removes a node with a given arena index
        pub fn remove_node(&mut self, index: usize) {

            // Deleting from arena
            if let Some(node) = self.arena.get_mut(index) {
                node.take();
            }
            // Deleting from parent node
            for node in self.arena.iter_mut() {
                if let Some(node) = node {
                    node.connected.retain(|i| i.unwrap() != index);
                }
            }

        }

        // Function gets the node from the graph (borrows it)
        pub fn get_node(&self, index: usize) -> Option<&Node<T>> {
            if let Some(node) = self.arena.get(index) {
                node.as_ref()
            } else {
                None
            }
        }

        // Function gets a mutable node from the graph (mutably borrows it)
        pub fn get_node_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
            if let Some(node) = self.arena.get_mut(index){
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


    // Separate implementation for Display bound
    impl<T: Display> Tree<T>{
        // Function prints the graph
        pub fn print(&mut self) {
            // Create an iterator of a tree
            let mut graph_iter = self.iter();
            
            println!("\nThe root of a graph is node {:?}", self.root.unwrap());

            // Iterate over the tree and print it's nodes
            while let Some(i) = graph_iter.next(&self) {
                let node = self.get_node(i).expect("Node not found!");
                node.print_node()    
            }          
        }

    }

}


// Module of a custom iterator
// Built-in Iterator trait doesn't fit current task. It can't mutably borrow an element of a vector if
// the vector has been borrowed already
mod iter{

    use super::tree::Tree;
    use std::fmt::Display;


    pub struct GraphIter{
        // Node arena indexes are stored on the stack
        stack: Vec<usize>
    }

    impl GraphIter{
        // Constructor of the iterator 
        pub fn new(root: Option<usize>) -> Self {
            // If there is a root - stack starts with it
            if let Some(root) = root {
                GraphIter {
                    stack: vec![root]
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
        pub fn next<T: Display>(&mut self, tree: &Tree<T>) -> Option<usize> {
            // Get the next index from the stack
            while let Some(node_index) = self.stack.pop(){
                // Get the node with that index from the graph (from arena)
                if let Some(node) = tree.get_node(node_index) {
                    // Add it's neighbours to the stack
                    let connected = &node.connected;
                    // Push each connected node onto the stack
                    for node in connected.iter(){
                            if let Some(node) = node {
                                // TODO get rid of dereferencing here?
                                self.stack.push(*node);
                            }
                    }

                    return Some(node_index)
                }
            }

            return None
        }
    }
}

