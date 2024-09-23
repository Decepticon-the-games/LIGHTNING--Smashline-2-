use super::*;

unsafe extern "C" fn game_specialsqigong(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    
    if macros::IS_GENERATABLE_ARTICLE(fighter, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG) {
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, false, -1);
            whiff_cancel(fighter);
        }
    }
}    

pub fn install() {
    Agent::new("lucario")
        .game_acmd("game_specialsqigong", game_specialsqigong, Priority::Low)
        .install();
}
