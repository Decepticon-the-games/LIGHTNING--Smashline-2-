use super::*;

unsafe extern "C" fn game_landingairlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 120, 0, 60, 6.0, 0.0, 5.0, 6.5, Some(0.0), Some(5.0), Some(-6.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("tantan")
        .game_acmd("game_landingairlw", game_landingairlw, Priority::Low)
        .install();
}
