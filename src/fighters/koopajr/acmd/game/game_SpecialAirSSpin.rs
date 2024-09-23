use super::*;

unsafe extern "C" fn game_specialairsspin(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_SPIN_TURN_JUMP);
        enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.0, 361, 80, 0, 74, 5.2, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_NORMAL_GRAVITY);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
}    

pub fn install() {
    Agent::new("koopajr")
        .game_acmd("game_specialairsspin", game_specialairsspin, Priority::Low)
        .install();
}
