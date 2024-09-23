use super::*;

unsafe extern "C" fn game_specialairnshoot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_N_FLAG_SHOOT);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_PARALYZER_BULLET, false, -1);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("szerosuit")
        .game_acmd("game_specialairnshoot", game_specialairnshoot, Priority::Low)
        .install();
}
