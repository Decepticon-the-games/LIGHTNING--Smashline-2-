use super::*;

unsafe extern "C" fn game_specialairs(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 19.0);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_S_WORK_ID_FLAG_ALREADY_EXIST_CRASHBOMB) {
        if macros::is_excute(fighter) {
            ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CRASHBOMB, false, -1);
            whiff_cancel(fighter);
        }
    }
}    

pub fn install() {
    Agent::new("rockman")
        .game_acmd("game_specialairs", game_specialairs, Priority::Low)
        .install();
}
