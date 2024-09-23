use super::*;

unsafe extern "C" fn game_specialairn2(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 100, 40, 0, 3.2, 0.0, 6.4, 2.2, Some(0.0), Some(6.4), Some(5.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_MAGIC);
    }
    frame(fighter.lua_state_agent, 24.0);
    macros::FT_MOTION_RATE(fighter, 1.25);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        WorkModule::set_float(fighter.module_accessor, 4.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_CHANGE_MAGIC);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::set_float(fighter.module_accessor, 16.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::set_float(fighter.module_accessor, -8.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::set_float(fighter.module_accessor, 24.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::set_float(fighter.module_accessor, -2.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::set_float(fighter.module_accessor, 12.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::set_float(fighter.module_accessor, -14.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_LAST_SHOOT);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_ICE, false, -1);
        whiff_cancel(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    wait(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.8, 42, 64, 0, 52, 3.2, 0.0, 6.4, 2.2, Some(0.0), Some(6.4), Some(6.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
    }
    wait(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(fighter.lua_state_agent, 57.0);
    macros::FT_START_ADJUST_MOTION_FRAME_arg1(fighter, 1.0);
}    

pub fn install() {
    Agent::new("trail")
        .game_acmd("game_specialairn2", game_specialairn2, Priority::Low)
        .install();
}
