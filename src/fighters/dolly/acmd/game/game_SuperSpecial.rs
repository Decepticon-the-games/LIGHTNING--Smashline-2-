use super::*;

unsafe extern "C" fn game_superspecial(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 4.0);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 5);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DOLLY_GENERATE_ARTICLE_BURST, false, -1);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 70.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 0.0, 8.0);
    }
}    

pub fn install() {
    Agent::new("dolly")
        .game_acmd("game_superspecial", game_superspecial, Priority::Low)
        .install();
}
