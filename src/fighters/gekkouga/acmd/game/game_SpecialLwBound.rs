use super::*;

unsafe extern "C" fn game_speciallwbound(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    macros::FT_MOTION_RATE(fighter, 1.2);
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_LW_FLAG_ENABLE_CONTROL);
    }
}    

pub fn install() {
    Agent::new("gekkouga")
        .game_acmd("game_speciallwbound", game_speciallwbound, Priority::Low)
        .install();
}
