use super::*;

unsafe extern "C" fn game_specialhiend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, DEADFALL);

    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_HULAHOOP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}    

pub fn install() {
    Agent::new("wiifit")
        .game_acmd("game_specialhiend", game_specialhiend, Priority::Low)
        .install();
}
