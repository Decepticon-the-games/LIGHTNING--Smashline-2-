use super::*;

unsafe extern "C" fn game_specialn_lb(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE, false, -1);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x25813802b6));
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
} 

pub fn install() {
    Agent::new("cloud")
        .game_acmd("game_specialn_lb", game_specialn_lb, Priority::Low)
        .install();
}
