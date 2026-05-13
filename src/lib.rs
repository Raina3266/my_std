#![cfg_attr(not(test), no_std)]
#![feature(allocator_api)]
#![deny(clippy::missing_safety_doc)]

// 3 crates: 
// core: numbers,
// alloc: vec, box, btreemap, etc. (but not hashmap)
// std: files, network, hashmap

extern crate alloc; // allow using alloc
pub mod my_box;
pub mod my_box_raina;
mod my_vec;