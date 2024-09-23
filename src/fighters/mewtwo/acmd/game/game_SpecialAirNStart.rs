use super::*;

unsafe extern "C" fn game_specialairnstart(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_SHADOWBALL, false, -1);
    }
}

pub fn install() {
    Agent::new("mewtwo")
        .game_acmd("game_specialairnstart", game_specialairnstart, Priority::Low)
        .install();
}
