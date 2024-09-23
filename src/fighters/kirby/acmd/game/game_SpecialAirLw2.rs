use super::*;

unsafe extern "C" fn game_specialairlw2(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;


    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_air_lw2"), false, -1.0);
    }
}

pub fn install() {
    Agent::new("kirby")
        .game_acmd("game_specialairlw2", game_specialairlw2, Priority::Low)
        .install();
}
