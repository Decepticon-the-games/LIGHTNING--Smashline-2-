use super::*;


use super::*;
unsafe extern "C" fn game_landingairlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 70, 0, 70, 2.5, 0.0, 2.7, 6.5, Some(0.0), Some(2.7), Some(-6.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("swordl1"), 7.0, 53, 73, 0, 58, 2.0, 3.0, -2.0, 0.0, Some(24.0), Some(-2.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("swordl1"), 7.0, 53, 73, 0, 58, 2.0, 3.0, -2.0, 0.0, Some(24.0), Some(-2.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 1.7);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
        AttackModule::clear(fighter.module_accessor, 2, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("edge")
        .game_acmd("game_landingairlw", game_landingairlw, Priority::Low)
        .install();
}
