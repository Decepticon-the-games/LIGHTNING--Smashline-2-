use super::*;

unsafe extern "C" fn game_specialnshoot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_DENGEKIDAMA, false, -1);
        whiff_cancel(fighter);
    }
} 

pub fn install() {
    Agent::new("pikachu")
        .game_acmd("game_specialnshoot", game_specialnshoot, Priority::Low)
        .install();
}
