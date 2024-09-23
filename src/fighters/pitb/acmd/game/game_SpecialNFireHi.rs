use super::*;

unsafe extern "C" fn game_specialnfirehi(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("pitb")
        .game_acmd("game_specialnfirehi", game_specialnfirehi, Priority::Low)
        .install();
}
