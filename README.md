___
This is a library to create / change / serialize / deserialize generic graphs
___
To take a look at it's functionality just run the `main.rs` file that contains a basic demo.

### Contents
- `node.rs`: Functionality of each node of the graph. It is important to mention that node has a generic value. 
- `graph.rs`: Functionality of a graph of nodes. Allows for adding, changing, deleting nodes and edges between the nodes.
- `iterator.rs`: Functionality of iterator of a graph. Implements _Breadth-First Search_ and a _Depth-First Search_ algorithms.
- `handler.rs`: Functionality of serializing and deserializing the graph. Allows to write the graph into the file and create a new graph from the given file.
- `lib.rs`: Exports all above. Contains Unit-tests.

### Important Details
- Serialization is done with [Trivial Graph Format](https://en.wikipedia.org/wiki/Trivial_Graph_Format)  
  (bold characters are written into the file)   
   __1 Root__    (index of a node and a label telling that this is a root)  
   __2__         (index of a node)  
   __3__         (index of a node)  
   __4__         (index of a node)  
   __'#'__       (delimeter between section of nodes and edges)  
   __1 2__       (indexes of nodes that have an edge between them)  
   __1 3__       (indexes of nodes that have an edge between them)  
   __3 4__       (indexes of nodes that have an edge between them)  
  
- Deserialization is done with the same format. But there are some things to notice here as well.   
  There are many modifications of a TGF and some of them regard the labels (text after the index of a node) as a value of a node. But this library is    intended to provide functionality for the __generic__ graph. That is, the node 
    can have value of any type (that __implements the `Default` trait__). But I haven't yet found a universal way to serialize any provided type into TGF string the way that after parsing a string representation of a node value from the file the compiler would know _what specific type a value should have_.    
    That means:   
   - __Only nodes' indexes and edges are written into the file and read from the file__.   
   - The __only label__ that has any effect is a `Root` label that indicates which 
      node is the root of the graph. __Any other__ labels are ignored.   
### Work Process
   - Create an empty graph with `Graph::new()`. Please, make sure to specify _graph's type_ in order for the program to work  
   - Add nodes with given values and/or connected nodes   
     __OR__   
     Deserialize nodes from the TGF file with `Handler::deserialize()`. Nodes will be added to the graph and edges between them will be created. But in this case you have to set __each node's value__ after deserialization.
   - Add edges (if you haven't connected any nodes on the previous step)
   - Set the root of the graph. Root is the node to start a graph traversal with
   - Change node values / change edges / print the graph etc.
   - Serialize the graph into the file (<ins>Warning</ins> all node values will be lost!)
   
