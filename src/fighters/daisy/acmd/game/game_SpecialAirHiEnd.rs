use super::*;

unsafe extern "C" fn game_specialairhiend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, Hash40::new("special_air_hi_end"), false, -1.0);
        whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("daisy")
        .game_acmd("game_specialairhiend", game_specialairhiend, Priority::Low)
        .install();
}
