use super::*;

unsafe extern "C" fn game_landingairf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 45, 160, 0, 50, 4.0, 0.0, 5.5, 10.0, Some(0.0), Some(5.5), Some(5.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("falco")
        .game_acmd("game_landingairf", game_landingairf, Priority::Low)
        .install();
}
