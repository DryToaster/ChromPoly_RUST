use std::io;

extern crate matrix;
use matrix::prelude::*;

let random_graph = Conventional::from_vec((4,4),[
	1.0, 0.0, 1.1, 0.0,
	1.0, 0.0, 1.1, 1.1,
	1.0, 1.0, 0.0, 1.1,
	0.0, 1.0, 1.0, 0.0,
]);

struct Graph {
	adjacency_matrix: Conventional,
	size: u32,
	order: u32,
}
impl Graph {
	fn build_graph(adj: Conventiona;) -> Graph {
	adjacenecy_matrix: Conventional,
	size: matrix.
	}
	fn is_complete() -> bool {}
	fn is_empty() -> bool {}
	fn is_null() -> bool {}
	fn chroma_decompose(x: u32, y:u32) -> (Graph, Graph) {}
	
}
 
