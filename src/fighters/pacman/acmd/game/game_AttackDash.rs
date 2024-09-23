use super::*;

unsafe extern "C" fn game_attackdash(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 1, Hash40::new("pizzapacman"), 0.0, 361, 100, 12, 0, 5.0, 0.0, 3.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 7.0);
    for _ in 0..3 {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 2.0, 12, 15, 0, 45, 5.5, 0.0, 5.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 7.0);
}
if macros::is_excute(fighter) {
    enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 4.0, 60, 65, 0, 100, 7.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
}
wait(fighter.lua_state_agent, 1.0);
if macros::is_excute(fighter) {
   AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
}
}    

pub fn install() {
    Agent::new("pacman")
        .game_acmd("game_attackdash", game_attackdash, Priority::Low)
        .install();
}
