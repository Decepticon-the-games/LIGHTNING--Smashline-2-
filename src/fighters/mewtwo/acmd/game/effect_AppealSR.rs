use super::*;

unsafe extern "C" fn effect_appealsr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_appeal_aura"), Hash40::new("top"), 0, -1.5, 0, 0, 0, 0, 1, true);
        macros::FLASH(fighter, 1, 0.7, 1, 0.5);
    }
    frame(fighter.lua_state_agent, 20.0);
    for _ in 0..7 {
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 20, 0, 15, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 10.0);
}
frame(fighter.lua_state_agent, 60.0);
if macros::is_excute(fighter) {
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_shield_smoke"), false, false);
}
}

pub fn install() {
    Agent::new("mewtwo")
        .game_acmd("effect_appealsr", effect_appealsr, Priority::Low)
        .install();
}
