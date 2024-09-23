use super::*;

unsafe extern "C" fn game_specialairs(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    macros::FT_MOTION_RATE(fighter, 1.2);
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_EXPLOSIVEFLAME, false, -1);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("palutena")
        .game_acmd("game_specialairs", game_specialairs, Priority::Low)
        .install();
}
