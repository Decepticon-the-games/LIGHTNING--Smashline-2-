use super::*;

unsafe extern "C" fn game_attacks3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::ATTACK(fighter, 1, 0, Hash40::new("stick"), 6.0, 90, 100, 25, 0, 4.1, 0.0, 5.5, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 0, 0, Hash40::new("stick"), 6.0, 45, 100, 45, 0, 3.0, 0.0, -5.5, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("stick"), 6.0, 285, 100, 25, 0, 4.1, 0.0, 5.5, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 0, 0, Hash40::new("stick"), 6.0, 100, 100, 40, 0, 3.0, 0.0, -5.5, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("stick"), 6.0, 20, 100, 35, 0, 4.1, 0.0, 5.5, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 0, 0, Hash40::new("stick"), 6.0, 85, 100, 30, 0, 3.0, 0.0, -5.5, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter); 
        frame(fighter.lua_state_agent, 25.0);
        macros::ATTACK(fighter, 1, 1, Hash40::new("stick"), 7.0, 40, 80, 0, 48, 4.8, 0.0, 5.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 0, 1, Hash40::new("stick"), 7.0, 40, 80, 0, 48, 3.9, 0.0, -5.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(fighter.lua_state_agent, 41.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
    }
}    

pub fn install() {
    Agent::new("palutena")
        .game_acmd("game_attacks3", game_attacks3, Priority::Low)
        .install();
}
