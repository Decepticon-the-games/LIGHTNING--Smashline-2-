use super::*;

unsafe extern "C" fn game_attackairf(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 1.0, 70, 10, 0, 90, 5.5, 1.0, -1.0, 0.0, None, None, None, 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 1.0, 70, 10, 0, 50, 5.5, 1.0, -1.0, 0.0, None, None, None, 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 1.0, 36, 10, 0, 60, 3.5, -2.5, 0.5, 0.0, None, None, None, 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 1.0, 366, 10, 0, 20, 1.5, 1.7, -5.8, -2.0, Some(8.0), Some(-6.2), Some(-2.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("footl"), 1.0, 366, 10, 0, 20, 1.5, 1.7, -5.8, 2.0, Some(8.0), Some(-6.2), Some(2.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 4, 0, Hash40::new("footl"), 1.0, 100, 10, 0, 60, 5.8, 6.0, -1.0, 0.0, None, None, None, 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 5, 0, Hash40::new("footl"), 1.0, 86, 10, 0, 90, 4.5, 3.0, 6.5, 0.0, None, None, None, 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 1.0, 366, 10, 0, 20, 1.5, 1.7, -5.8, -2.0, Some(10.0), Some(-6.2), Some(-2.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("footl"), 1.0, 366, 10, 0, 20, 1.5, 1.7, -5.8, 2.0, Some(10.0), Some(-6.2), Some(2.0), 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 4.0, 361, 132, 0, 50, 9.0, 6.0, 1.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 78.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}    

pub fn install() {
    Agent::new("rosetta")
        .game_acmd("game_attackairf", game_attackairf, Priority::Low)
        .install();
}
