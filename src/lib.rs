#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

pub mod fighters;
pub mod utils;
pub mod dynamics;

#[skyline::main(name = "lightning")]
pub fn main() {
    fighters::install();
    utils::install();
    //dynamics::install();
}