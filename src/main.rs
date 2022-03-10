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
	let e = tree.add_node(Node::new("Working", vec![a, b, c, d]));

	// Set the root of a tree
	tree.set_root(Some(a));

	// Create an iterator of a tree
	let mut tree_iter = tree.iter();
	// Iterate over the tree and print it's nodes
	while let Some(i) = tree_iter.next(&tree) {
		let node = tree.get_node(i).expect("No such node!");
		println!("{}", node.value);
	}
}