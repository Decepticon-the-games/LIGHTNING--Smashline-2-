use super::*;

unsafe extern "C" fn game_attackhi3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 78, 18, 0, 72, 3.0, 0.0, 9.5, 4.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 78, 18, 0, 78, 3.0, 0.0, 8.0, 7.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 78, 18, 0, 72, 3.0, 0.0, 11.0, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 78, 18, 0, 78, 3.0, 0.0, 12.0, 8.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 78, 18, 0, 72, 3.0, 0.0, 19.0, 4.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 78, 18, 0, 78, 3.0, 0.0, 23.0, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
}    

pub fn install() {
    Agent::new("dolly")
        .game_acmd("game_attackhi3", game_attackhi3, Priority::Low)
        .install();
}
