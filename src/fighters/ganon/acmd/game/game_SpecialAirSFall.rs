use super::*;

unsafe extern "C" fn game_specialairsfall(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        macros::SET_SPEED_EX(fighter, 0, -5, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_GANON_EXPLOSION_FALL_SETTING_FALL, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_FALL_SETTING);
    }
}    

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_specialairsfall", game_specialairsfall, Priority::Low)
        .install();
}
