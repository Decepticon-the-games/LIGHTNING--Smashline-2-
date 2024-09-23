use super::*;

unsafe extern "C" fn game_specialhi1(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_TORNADO, false, -1);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_REVERT_ANGLE);
    }    
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("brave")
        .game_acmd("game_specialhi1", game_specialhi1, Priority::Low)
        .install();
}
