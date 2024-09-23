
use super::*;

pub mod crimson_cancel;
pub mod cross_cancel;
pub mod vanish;
pub mod lightning_mode;

pub fn install() {
    crimson_cancel::install();
    cross_cancel::install();
    vanish::install();
    lightning_mode::install();
}   