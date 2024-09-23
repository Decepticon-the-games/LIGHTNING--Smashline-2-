use super::*;

unsafe extern "C" fn game_specialhiattacklanding(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_HAMMER, false, -1);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove(fighter.module_accessor, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_HAMMER);
    }
}    

pub fn install() {
    Agent::new("koopajr")
        .game_acmd("game_specialhiattacklanding", game_specialhiattacklanding, Priority::Low)
        .install();
}
