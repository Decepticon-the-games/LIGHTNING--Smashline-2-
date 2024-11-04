#![feature(concat_idents, proc_macro_hygiene)]
#![allow(unused_macros)]
#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod fighters;
//pub mod utils;
pub mod dynamics;

#[skyline::main(name = "lightning")]
pub fn main() {
    fighters::install();
    //utils::install();
    //dynamics::install();
}