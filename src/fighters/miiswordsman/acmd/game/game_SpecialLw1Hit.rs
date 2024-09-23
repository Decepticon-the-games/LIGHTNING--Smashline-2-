use super::*;

unsafe extern "C" fn game_speciallw1hit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 60, 0, 85, 8.8, 0.0, 8.0, 15.0, Some(0.0), Some(8.0), Some(3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    macros::FT_MOTION_RATE(fighter, 0.65);
}    

pub fn install() {
    Agent::new("miiswordsman")
        .game_acmd("game_speciallw1hit", game_speciallw1hit, Priority::Low)
        .install();
}
