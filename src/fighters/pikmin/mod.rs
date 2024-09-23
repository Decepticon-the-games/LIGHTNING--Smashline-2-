
// use ::common::prelude::*;
use super::*;
pub static mut PIKMIN_PIKMIN_ATTACK_CANCEL : [bool; 8] = [false; 8];
pub static mut PIKMIN_ATTACK_CANCEL : [bool; 8] = [false; 8];

pub mod acmd;

//pub mod status;
//pub mod opff;
//pub mod params;


pub fn install() {
    acmd::install();
    //status::install();
    //opff::install();
    //params::install();
}