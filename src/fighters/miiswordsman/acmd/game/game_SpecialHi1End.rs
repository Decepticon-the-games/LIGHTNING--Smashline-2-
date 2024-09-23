use super::*;

unsafe extern "C" fn game_specialhi1end(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
     
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
    enable_attack_cancel(fighter);    
    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 55, 93, 0, 103, 8.0, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(4.5), 2.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("miiswordsman")
        .game_acmd("game_specialhi1end", game_specialhi1end, Priority::Low)
        .install();
}
