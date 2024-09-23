use super::*;

pub mod cancels;
pub mod lightning_mechanics;

pub fn install() {
    cancels::install();
    lightning_mechanics::install();
} 