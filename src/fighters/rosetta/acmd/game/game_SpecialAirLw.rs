use super::*;

unsafe extern "C" fn game_specialairlw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROSETTA_STATUS_SPECIAL_LW_FLAG_ENABLE_SEARCH);
    }
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROSETTA_STATUS_SPECIAL_LW_FLAG_ENABLE_SEARCH);
    }
    if macros::is_excute(fighter) {
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK);
        ItemModule::use_item(fighter.module_accessor, false, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
        ItemModule::pickup_item(fighter.module_accessor, *ITEM_SIZE_LIGHT);
    }
    wait(false, 1);
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("rosetta")
        .game_acmd("game_specialairlw", game_specialairlw, Priority::Low)
        .install();
}
