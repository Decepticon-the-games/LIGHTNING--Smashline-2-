use super::*;

unsafe extern "C" fn game_specialn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_DENGEKIDAMA, false, -1);
        whiff_cancel(fighter);
    }
    macros::FT_MOTION_RATE(fighter, 0.77);
}    

pub fn install() {
    Agent::new("pikachu")
        .game_acmd("game_specialn", game_specialn, Priority::Low)
        .install();
}
