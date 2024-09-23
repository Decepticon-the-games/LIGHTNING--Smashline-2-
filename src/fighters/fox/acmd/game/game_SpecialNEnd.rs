use super::*;

unsafe extern "C" fn game_specialnend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40::new("close"), false, -1.0);
            whiff_cancel(fighter);
        }
    }
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
}    

pub fn install() {
    Agent::new("fox")
        .game_acmd("game_specialnend", game_specialnend, Priority::Low)
        .install();
}
