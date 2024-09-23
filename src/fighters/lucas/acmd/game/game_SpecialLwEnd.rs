use super::*;

unsafe extern "C" fn game_speciallwend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 25, 98, 0, 40, 3.5, 0.0, 6.3, 7.0, Some(0.0), Some(6.3), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("lucas")
        .game_acmd("game_speciallwend", game_speciallwend, Priority::Low)
        .install();
}
