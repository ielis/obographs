//! The `obographs` crate defines the building blocks 
//! of the Obographs format and offers I/O routines 
//! for loading Obographs files.
//!
//! ## Examples
//! 
//! The Obographs elements can be created programatically.
//! For instance, and `Edge`:
//! 
//! ```
//! use obographs::model::*;
//! 
//! let edge = Edge {
//!   sub: String::from("http://purl.obolibrary.org/obo/HP_0001250"),
//!   pred: String::from("is_a"),
//!   obj: String::from("http://purl.obolibrary.org/obo/HP_0012638"),
//!   meta: None,
//! };
//! ```
//! 
//! 


mod io;
pub mod model;
