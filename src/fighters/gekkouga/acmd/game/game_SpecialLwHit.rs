use super::*;

unsafe extern "C" fn game_speciallwhit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_LW_FLAG_GENERATE_ITEM);
    }
}    

pub fn install() {
    Agent::new("gekkouga")
        .game_acmd("game_speciallwhit", game_speciallwhit, Priority::Low)
        .install();
}
