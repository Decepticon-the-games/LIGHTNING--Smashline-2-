use super::*;

unsafe extern "C" fn game_specialhi1(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 7.0, 107, 60, 0, 70, 3.0, 0.0, -2.0, 9.0, Some(0.0), Some(-2.0), Some(-4.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
        JostleModule::set_status(fighter.module_accessor, true);
    }
}    

pub fn install() {
    Agent::new("edge")
        .game_acmd("game_specialhi1", game_specialhi1, Priority::Low)
        .install();
}
