use super::*;

unsafe extern "C" fn game_attackstand31(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
       // JostleModule::set_push_speed_x_overlap_rate_status(fighter.module_accessor, 0.1);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 90, 120, 61, 0, 3.6, 0.0, 17.0, 11.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 90, 120, 61, 0, 4.2, 0.0, 17.0, 11.0, Some(0.0), Some(4.2), Some(3.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 90, 120, 61, 0, 4.0, 0.0, 12.0, 4.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 70, 120, 21, 0, 4.0, 0.0, 12.0, 4.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 7.0, 90, 120, 21, 0, 3.6, 0.0, 17.0, 11.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 7.0, 90, 120, 21, 0, 4.2, 0.0, 17.0, 11.0, Some(0.0), Some(4.2), Some(3.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, 8.0, false);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
       // JostleModule::set_push_speed_x_overlap_rate_status(fighter.module_accessor, 0);
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_3_FLAG_CHECK_STEP);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}  

pub fn install() {
    Agent::new("demon")
        .game_acmd("game_attackstand31", game_attackstand31, Priority::Low)
        .install();
}
