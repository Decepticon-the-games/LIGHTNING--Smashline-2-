use super::*;

unsafe extern "C" fn game_specialairs3_1hi(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    macros::FT_MOTION_RATE(fighter, 1.22);
}    

pub fn install() {
    Agent::new("miiswordsman")
        .game_acmd("game_specialairs3_1hi", game_specialairs3_1hi, Priority::Low)
        .install();
}
