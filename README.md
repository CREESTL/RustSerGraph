## Overview

__What is it?__ -- This is a library that allows you to create / change / serialize / deserialize generic graphs

To take a look at it's functionality just run the `main.rs` file that contains a basic demo

### Contents
- `node.rs`: Functionality of each node of the graph. It is important to mention that node has a generic value. 
- `graph.rs`: Functionality of a graph of nodes. Allows for adding, changind, deleting nodes and edges between the nodes.
- `iterator.rs`: Functionality of iterator of a graph. Implements _Breadth-First Search_ and a _Depth-First Search_ algorithms.
- `handler.rs`: Functionality of serializing and deserializing the graph. Allows to write the graph into the file and create a new graph from the given file.
- `lib.rs`: Exports all above. Contains Unit-tests.

### Important Details
1) Serialization is done with [Trivial Graph Format](https://en.wikipedia.org/wiki/Trivial_Graph_Format)  
   __Current Format__ (bold characters are written into the file)   
   __1 Root__    (index of a node and a label telling that this is a root)  
   __2__         (index of a node)  
   __3__         (index of a node)  
   __4__         (index of a node)  
   __'#'__       (delimeter between section of nodes and edges)  
   __1 2__       (numbers of nodes that have an edge between them)  
   __1 3__       (numbers of nodes that have an edge between them)  
   __3 4__       (numbers of nodes that have an edge between them)  
  
 2) Deserialization is done with the same format. But there are some things to notice here as well. There are many modifications of a TGF and some of them regard the 
    labels (text after the index of a node) as a value of a node. But this library is intended to provide functionality for the __generic__ graph. That is, the node 
    can have value of any type (that __implements the `Default` trait__). But I haven't yet found a universal way to serialize any provided type into TGF string. That means 
    that __only nodes' indexes and edges are written into the file and read from the file__. The only label that has any effect is a `Root` label that indicates which 
    node is the root of the graph. __Any other__ labels are ignored. If you create a graph you have to add each node into it and then add edges between the nodes. If your graph has been created after deserialization,
    you have to set each node's value.
   
