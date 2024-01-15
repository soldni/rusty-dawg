mod cdawg_edge_weight;
mod crochemore;  // Algo from "On Compact Directed Acyclic Word Graphs"
mod inenaga;  // Algo from "On-line construction of compact directed acyclic word graphs"
mod metadata;

// We will use the Inenaga implementation of the build algorithm.
pub use self::inenaga::Cdawg;