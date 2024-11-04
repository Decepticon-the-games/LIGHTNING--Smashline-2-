use {
    crate::{
        fighters::common::function_hooks::ui::{
            lightning_meter::*,
            ui_manager::*,
            ui_object::*,
            utility::*,
        },
        dynamics::util::*,
    },
    
    once_cell::sync::Lazy,
    parking_lot::RwLock
};

pub mod lightning_meter;
pub mod ui_hook;
pub mod ui_manager;
pub mod ui_object;
pub mod utility;

pub fn install() {
    ui_hook::install();
}