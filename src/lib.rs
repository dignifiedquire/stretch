#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

#[macro_use]
extern crate lazy_static;

#[cfg_attr(feature = "serde", macro_use)]
#[cfg(feature = "serde")]
extern crate serde;

pub mod geometry;
pub mod node;
pub mod number;
pub mod result;
pub mod style;

mod algo;
mod forest;
pub mod id;

pub use crate::forest::{NodeData, StretchNodeData};
pub use crate::node::Stretch;

use core::any::Any;

#[derive(Debug)]
pub enum Error<N: node::Node> {
    InvalidNode(N),
    Measure(Box<dyn Any>),
}

#[cfg(feature = "std")]
impl<N: node::Node> std::fmt::Display for Error<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::InvalidNode(ref node) => write!(f, "Invalid node {:?}", node),
            Error::Measure(_) => write!(f, "Error during measurement"),
        }
    }
}

#[cfg(feature = "std")]
impl<N: node::Node> std::error::Error for Error<N> {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidNode(_) => "The node is not part of the stretch instance",
            Error::Measure(_) => "Error occurred inside a measurement function",
        }
    }
}
