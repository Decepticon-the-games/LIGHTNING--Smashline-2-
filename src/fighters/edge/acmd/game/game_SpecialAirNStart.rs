use super::*;

unsafe extern "C" fn game_specialairnstart(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_EDGE_SPECIAL_N_S, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_EDGE_SPECIAL_N_M, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(fighter.lua_state_agent, 79.0);
    macros::FT_MOTION_RATE(fighter, 1.2);
    frame(fighter.lua_state_agent, 99.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 100.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_EDGE_SPECIAL_N_L, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(fighter.lua_state_agent, 105.0);
    macros::FT_MOTION_RATE(fighter, 1.6);
    frame(fighter.lua_state_agent, 115.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 120.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, -1);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 140.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
}    

pub fn install() {
    Agent::new("edge")
        .game_acmd("game_specialairnstart", game_specialairnstart, Priority::Low)
        .install();
}
