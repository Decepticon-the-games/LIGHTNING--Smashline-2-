use super::*;

unsafe extern "C" fn game_specialnend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_SEED, false, -1);
        whiff_cancel(fighter);
    }    
}

pub fn install() {
    Agent::new("pfushigisou")
        .game_acmd("game_specialnend", game_specialnend, Priority::Low)
        .install();
}