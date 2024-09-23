use super::*;

unsafe extern "C" fn game_throwhi(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.0, 82, 50, 0, 25, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("mouth2"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 21.0);
    for _ in 0..7 {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 0, 100, 0, 0, 9.6, 0.0, 20.0, 1.6, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
frame(fighter.lua_state_agent, 49.0);
if macros::is_excute(fighter) {
    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 40, 100, 80, 0, 9.6, 0.0, 20.0, 1.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
}
frame(fighter.lua_state_agent, 50.0);
if macros::is_excute(fighter) {
    HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
}
frame(fighter.lua_state_agent, 52.0);
if macros::is_excute(fighter) {
    AttackModule::clear_all(fighter.module_accessor);
}
frame(fighter.lua_state_agent, 57.0);
if macros::is_excute(fighter) {
    enable_attack_cancel(fighter); 
    macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 4.0, 80, 80, 0, 85, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
}
wait(fighter.lua_state_agent, 2.0);
if macros::is_excute(fighter) {
    AttackModule::clear_all(fighter.module_accessor);
    whiff_cancel(fighter);
}
macros::FT_MOTION_RATE(fighter, 1.0);
}

pub fn install() {
    Agent::new("koopa")
        .game_acmd("game_throwhi", game_throwhi, Priority::Low)
        .install();
}
