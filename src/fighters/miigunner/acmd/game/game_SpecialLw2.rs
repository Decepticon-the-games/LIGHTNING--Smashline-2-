use super::*;

unsafe extern "C" fn game_speciallw2(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.15);
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_GROUND_BOMB_FLAG_EXIST_BOMB_BURST);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GROUNDBOMB, false, -1);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("miigunner")
        .game_acmd("game_speciallw2", game_speciallw2, Priority::Low)
        .install();
}
