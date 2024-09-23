use super::*;

unsafe extern "C" fn game_attackairlw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;    

    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    for _ in 0..6 {
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.3, 367, 80, 0, 5, 5.0, 0.0, 1.0, 2.0, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.8, 367, 80, 0, 5, 3.5, 0.0, 2.0, 6.5, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.8, 367, 80, 0, 5, 3.5, 0.0, 2.0, -2.5, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
}
for _ in 0..7 {
wait(fighter.lua_state_agent, 1.0);
if macros::is_excute(fighter) {
    AttackModule::clear_all(fighter.module_accessor);
}
wait(fighter.lua_state_agent, 1.0);
if macros::is_excute(fighter) {
    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.9, 367, 100, 0, 5, 5.0, 0.0, 0.5, 2.0, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.4, 367, 100, 0, 5, 3.5, 0.0, 1.5, 6.5, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.4, 367, 100, 0, 5, 3.5, 0.0, 1.5, -2.5, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
}
}
if macros::is_excute(fighter) {
AttackModule::clear_all(fighter.module_accessor);
}
wait(fighter.lua_state_agent, 1.0);
if macros::is_excute(fighter) {
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.8, 80, 179, 0, 55, 6.5, 0.0, 0.5, 2.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.8, 40, 179, 0, 55, 4.5, 0.0, 1.5, 6.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.8, 40, 179, 0, 55, 4.5, 0.0, 1.5, -2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
enable_attack_cancel(fighter); 
}

wait(fighter.lua_state_agent, 1.0);
if macros::is_excute(fighter) {
AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
}
frame(fighter.lua_state_agent, 50.0);
if macros::is_excute(fighter) {
WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
}
}    

pub fn install() {
    Agent::new("yoshi")
        .game_acmd("game_attackairlw", game_attackairlw, Priority::Low)
        .install();
}
