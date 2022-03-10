#![allow(dead_code)]

// Export structs
pub use node::Node;
pub use tree::Tree;
pub use iter::GraphIter;


// Module of a graph node
mod node {
    // Alias ti separate index of a node and index of a node in the arena vector
    type Number = usize;

    // Struct of a graph node
    pub struct Node<T> {
        // Node has an index, a value and may have other nodes connected to it
        // Index is equal to node's position in arena of a graph.
        pub index: Option<usize>,
        pub value: T,
        pub connected: Vec<Option<Node<T>>>
    }

    impl<T> Node<T> {
        // Constructor for a new node
        pub fn new(value: T, connected: Vec<Option<Node<T>>>) -> Self{
            // By default index of a node is None
            // Changes after a node is added to the arena
            Node{index: None, value, connected}
        }
    }
}


// Module of a graph
mod tree {
    use super::node::Node;
    use super::iter::GraphIter;

    // Struct of a graph
    pub struct Tree<T> {
        // Graph has a root and an arena
        // Arena is a vector holding nodes of a graph. Allows for random access without nested borrowing
        // Access to each node from arena is through it's index.
        arena: Vec<Option<Node<T>>>,
        // Root is one of the nodes in arena. Access through index as well.
        root: Option<usize>,
    }

    impl<T> Tree<T>{
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
            let mut node = self.arena.iter_mut()
            .filter_map(Option::as_mut)
            .find(|node| node.index == Some(index));
            node.take();
        }

        // Function gets the node from the graph (borrows it)
        pub fn get_node(&self, index: usize) -> Option<&Node<T>> {
            let node = self.arena.iter()
            .filter_map(Option::as_ref)
            .find(|node| node.index == Some(index));

            node
        }

        // Function gets a mutable node from the graph (mutably borrows it)
        pub fn get_node_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
            let node = self.arena.iter_mut()
            .filter_map(Option::as_mut)
            .find(|node| node.index == Some(index));

            node
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

    use super::tree::Tree;

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
        pub fn next<T>(&mut self, tree: &Tree<T>) -> Option<usize> {
            // Get the next index from the stack
            while let Some(node_index) = self.stack.pop(){
                // Get the node with that index from the graph (from arena)
                if let Some(node) = tree.get_node(node_index) {
                    // Add it's neighbours to the stack
                    let connected = &node.connected;
                    // Push each connected node onto the stack
                    for node in connected.iter(){
                            if let Some(node) = node {
                                // Use unwrap_or? Can it be that there is no index?
                                self.stack.push(node.index.unwrap());
                            }
                    }

                    return Some(node_index)
                }
            }

            return None
        }
    }
}

