//! Pure Rust implementation of LZ4 compression.
//!
//! A detailed explanation of the algorithm can be found [here](http://ticki.github.io/blog/how-lz4-works/).

extern crate byteorder;
#[macro_use]
extern crate quick_error;

pub mod block;
pub mod frame;
#[cfg(test)]
mod tests;

#[cfg(test)]
#[macro_use] 
extern crate more_asserts;

pub use frame::compress::{compress as frame_compress};
pub use block::compress::{compress, compress_into};
pub use block::decompress::{decompress, decompress_into};
