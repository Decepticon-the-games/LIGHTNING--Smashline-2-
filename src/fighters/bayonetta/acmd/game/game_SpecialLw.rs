use super::*;
//#[acmd_script( fighter = "bayonetta", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn game_speciallw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 11.5, -2.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), false);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
    }
}    
pub fn install() {
Agent::new("bayonetta")
    .game_acmd("game_speciallw", game_speciallw, Priority::Low)
    .install();
}