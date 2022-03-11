mod lib;
use lib::{Node, Tree};

fn main() {
	// TODO place deserialization from file here

	// Create a simple tree with 5 nodes
	let mut tree = Tree::new();
	let a = tree.add_node(Node::new("Simple", vec![]));
	let b = tree.add_node(Node::new("Text", vec![]));
	let c = tree.add_node(Node::new("Generic", vec![]));
	let d = tree.add_node(Node::new("Is", vec![]));
	let e = tree.add_node(Node::new("Working", vec![Some(a), Some(b), Some(c), Some(d)]));

	// Set the root of a tree
	tree.set_root(Some(e));

	tree.print();

	tree.remove_node(2);

	tree.print();

}