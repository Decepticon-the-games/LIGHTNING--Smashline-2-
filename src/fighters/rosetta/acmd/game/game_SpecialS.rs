use super::*;

unsafe extern "C" fn game_specials(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_ROSETTA_TICO_STATUS_SPECIAL_S_WORK_FLAG_SHOOT);
    }

    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("rosetta_tico")
        .game_acmd("game_specials", game_specials, Priority::Low)
        .install();
}
