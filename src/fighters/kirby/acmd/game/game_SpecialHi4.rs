use super::*;

unsafe extern "C" fn game_specialhi4(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;


    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi4"), false, -1.0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTERSHOT, false, -1);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("kirby")
        .game_acmd("game_specialhi4", game_specialhi4, Priority::Low)
        .install();
}
