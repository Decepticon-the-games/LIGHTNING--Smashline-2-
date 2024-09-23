use super::*;

unsafe extern "C" fn game_specialnuse2(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        item(*MA_MSC_CMD_ITEM_THROW_ITEM_OFFSET_MOTION, 10, 12, Hash40::new("item_heavy_throw_f"));
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("shizue")
        .game_acmd("game_specialnuse2", game_specialnuse2, Priority::Low)
        .install();
}
