//! It solves the so-called [`dynamic connectivity problem`]. The lib provides three algorithms:
//! - Quick Find
//! - Quick Union
//! - TODO: Weighting
//!
//! [`dynamic connectivity problem`]: https://en.wikipedia.org/wiki/Dynamic_connectivity
//!
//! Given a set on N objects:
//! - Union command: connect two objects
//! - Find/connected query: is there a path connecting the two objects?
//! 
//! Connected components: Maximal set of objects that are mutually connected. Here an example of 3 connected components.
//! 
//! {0}, {1 4 5}, {2 3 6 7}
//! 
//! Find Query op check if two objects are in the same component while union command op replace components containing two objects with their union.
//! 

/// Eager approach
pub mod quickfind;

pub mod quickunion;

pub mod weighting;