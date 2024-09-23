use super::*;
//#[acmd_script( fighter = "bayonetta", script = "game_landingairlw", category = ACMD_GAME, low_priority )]
use super::*;
unsafe extern "C" fn game_landingairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLAG_LANDING_DISABLE_ATTACK) {
    }
    else{
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter); 
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, false, 10, 3, 3, 0, true);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 45, 127, 0, 80, 7.5, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 45, 127, 0, 80, 2.5, 0.0, 2.5, 12.0, Some(0.0), Some(2.5), Some(16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
}
wait(fighter.lua_state_agent, 1.0);
if macros::is_excute(fighter) {
    AttackModule::clear(fighter.module_accessor, 1, false);
}
wait(fighter.lua_state_agent, 1.0);
if macros::is_excute(fighter) {
    AttackModule::clear_all(fighter.module_accessor);
    whiff_cancel(fighter);
}
frame(fighter.lua_state_agent, 5.0);
if macros::is_excute(fighter) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
}
}
pub fn install() {
Agent::new("bayonetta")
    .game_acmd("game_landingairlw",game_landingairlw, Priority::Low)
    .install();
}