use super::*;

unsafe extern "C" fn game_specialairn3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_INT_THUNDER_NUM);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, false, -1);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_FLAG_CHANGE_MAGIC);
    }
    wait(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_INT_THUNDER_NUM);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, false, -1);
    }
    wait(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_INT_THUNDER_NUM);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, false, -1);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(fighter.lua_state_agent, 70.0);
    if macros::is_excute(fighter) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}    

pub fn install() {
    Agent::new("trail")
        .game_acmd("game_specialairn3", game_specialairn3, Priority::Low)
        .install();
}
