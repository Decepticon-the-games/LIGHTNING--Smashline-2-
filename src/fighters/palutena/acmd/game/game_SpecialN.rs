use super::*;

unsafe extern "C" fn game_specialn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::SEARCH(fighter, 0, 0, Hash40::new("bust"), 120.0, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("palutena")
        .game_acmd("game_specialn", game_specialn, Priority::Low)
        .install();
}
