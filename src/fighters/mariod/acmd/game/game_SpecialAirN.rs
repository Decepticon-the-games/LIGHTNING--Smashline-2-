use super::*;

unsafe extern "C" fn game_specialairn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    macros::FT_MOTION_RATE(fighter, 1.15);
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, false, -1);
    }
}

pub fn install() {
    Agent::new("mariod")
        .game_acmd("game_specialairn", game_specialairn, Priority::Low)
        .install();
}
