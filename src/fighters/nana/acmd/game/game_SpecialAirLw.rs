use super::*;

unsafe extern "C" fn game_specialairlw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 16.0);
    for _ in 0..9 {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_BLIZZARD, false, -1);
    }
    wait(fighter.lua_state_agent, 5.0);
}
frame(fighter.lua_state_agent, 70.0);
if macros::is_excute(fighter) {
    whiff_cancel(fighter);
}
}

pub fn install() {
    Agent::new("nana")
        .game_acmd("game_specialairlw", game_specialairlw, Priority::Low)
        .install();
}
