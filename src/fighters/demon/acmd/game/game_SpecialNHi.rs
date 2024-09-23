use super::*;

unsafe extern "C" fn game_specialnhi(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, false, -1);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        whiff_cancel(fighter);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_SCALING);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_SCALING);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 60.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 64.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 65.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 66.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 67.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}    

pub fn install() {
    Agent::new("demon")
        .game_acmd("game_specialnhi", game_specialnhi, Priority::Low)
        .install();
}
