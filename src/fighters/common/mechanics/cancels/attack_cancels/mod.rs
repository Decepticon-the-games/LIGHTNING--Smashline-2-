use super::*;

pub mod cancel_on_hit;
//pub mod resets_falses;

pub fn install() {
    //resets_falses::install();
    cancel_on_hit::install();
}
