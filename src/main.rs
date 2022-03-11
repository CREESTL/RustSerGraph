mod lib;
use lib::{Node, Tree};

fn main() {
	// TODO place deserialization from file here

	// Create a simple tree with 5 nodes
	let mut tree = Tree::new();
	tree.add_node(Node::new(1,"Simple", vec![]));
	tree.add_node(Node::new(2,"Text", vec![]));
	tree.add_node(Node::new(3,"Generic", vec![]));
	tree.add_node(Node::new(4,"Is", vec![]));
	tree.add_node(Node::new(666,"Working", vec![1, 2, 3, 4]));

	// Set the root of a tree
	tree.set_root(Some(666));

	tree.print();

	tree.remove_node(2);

}