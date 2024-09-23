use super::*;

unsafe extern "C" fn game_specialairhi2(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(fighter) {
            ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(fighter.module_accessor) {
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_SPREADBULLET, false, -1);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_SPREADBULLET, false, -1);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_SPREADBULLET, false, -1);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_SPREADBULLET, false, -1);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_SPREADBULLET, false, -1);
        //whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(fighter) {
            ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(fighter.module_accessor) {
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
}

pub fn install() {
    Agent::new("elight")
        .game_acmd("game_specialairhi2", game_specialairhi2, Priority::Low)
        .install();
}
