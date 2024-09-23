use super::*;

unsafe extern "C" fn game_specialhi(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_GIMMICKJUMP, *ARTICLE_OPE_TARGET_ALL, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}    

pub fn install() {
    Agent::new("sonic")
        .game_acmd("game_specialhi", game_specialhi, Priority::Low)
        .install();
}
