use super::*;

unsafe extern "C" fn game_specialn1(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_FIRE, false, -1);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_N1_FLAG_CAN_INPUT);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_N1_FLAG_CHANGE_MAGIC);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_N1_FLAG_CAN_INPUT);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
}    

pub fn install() {
    Agent::new("trail")
        .game_acmd("game_specialn1", game_specialn1, Priority::Low)
        .install();
}
