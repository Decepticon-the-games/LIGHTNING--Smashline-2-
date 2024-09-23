use super::*;

unsafe extern "C" fn game_throwhi(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.0, 90, 110, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 2, 20);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.6);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 4.0, z: 0.0});
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
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
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40::new("close"), false, -1.0);
        }
    }
}    

pub fn install() {
    Agent::new("fox")
        .game_acmd("game_throwhi", game_throwhi, Priority::Low)
        .install();
}
