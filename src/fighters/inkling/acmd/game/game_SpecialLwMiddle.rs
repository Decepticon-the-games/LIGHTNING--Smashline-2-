use super::*;

unsafe extern "C" fn game_speciallwmiddle(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SPLASHBOMB, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_STATUS_SPECIAL_LW_FLAG_THROW);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
}    

pub fn install() {
    Agent::new("inkling")
        .game_acmd("game_speciallwmiddle", game_speciallwmiddle, Priority::Low)
        .install();
}
