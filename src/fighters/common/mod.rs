use super::*;

pub mod mechanics;
pub mod function_hooks;
pub mod general;

pub fn install() {

    general::install();
    mechanics::install();
    function_hooks::install();

}