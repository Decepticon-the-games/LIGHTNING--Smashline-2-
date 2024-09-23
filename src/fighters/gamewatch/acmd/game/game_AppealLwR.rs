use super::*;

unsafe extern "C" fn game_appeallwr(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::set_int(fighter.module_accessor, *WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_SPRAY, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("appeal_lw"), false, -1.0);
    }
}    

pub fn install() {
    Agent::new("gamewatch")
        .game_acmd("game_appeallwr", game_appeallwr, Priority::Low)
        .install();
}
