use super::*;

unsafe extern "C" fn game_throwf(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 361, 100, 30, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SHOULDER_START_FLAG_SHOULDER_WAIT);
    }
}    

pub fn install() {
    Agent::new("donkey")
        .game_acmd("game_throwf", game_throwf, Priority::Low)
        .install();
}
