use super::*;

unsafe extern "C" fn game_attackstand21(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 0, 50, 60, 0, 4.0, 0.0, 11.0, 10.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 0, 50, 60, 0, 4.0, 0.0, 12.0, 2.0, Some(0.0), Some(11.5), Some(7.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 10, 40, 100, 0, 4.0, 0.0, 14.0, 3.0, Some(0.0), Some(13.0), Some(9.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 10.0, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STAND_2_FLAG_CHECK_STEP);
    }
}  

pub fn install() {
    Agent::new("demon")
        .game_acmd("game_attackstand21", game_attackstand21, Priority::Low)
        .install();
}
