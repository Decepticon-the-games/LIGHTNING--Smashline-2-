use super::*;


use super::*;
unsafe extern "C" fn game_landingairlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter);
        macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 1.0, 45, 150, 0, 20, 4.0, 0.0, 4.0, 3.0, Some(0.0), Some(4.0), Some(-3.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
    }
}


pub fn install() {
    Agent::new("fox")
        .game_acmd("game_landingairlw", game_landingairlw, Priority::Low)
        .install();
}
