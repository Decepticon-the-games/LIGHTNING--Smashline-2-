use super::*;

unsafe extern "C" fn game_throwb(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.0, 56, 85, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        macros::CHECK_FINISH_CAMERA(fighter, 8, 6);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 3.0, y: 1.0, z: 0.0});
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40::new("open"), false, -1.0);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        whiff_cancel(fighter);
    }
    
    frame(fighter.lua_state_agent, 24.0);
    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40::new("close"), false, -1.0);
        }
    }
}    

pub fn install() {
    Agent::new("fox")
        .game_acmd("game_throwb", game_throwb, Priority::Low)
        .install();
}
