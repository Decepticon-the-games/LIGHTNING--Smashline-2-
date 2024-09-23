use super::*;

unsafe extern "C" fn game_attackhi4(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter); 
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 15.0, 93, 88, 0, 52, 6.1, 2.9, 0.0, 0.0, None, None, None, 1.3, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 100, 0, 38, 3.4, 0.0, 5.2, -7.0, Some(0.0), Some(5.2), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    whiff_cancel(fighter);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

pub fn install() {
    Agent::new("sheik")
        .game_acmd("game_attackhi4", game_attackhi4, Priority::Low)
        .install();
}
