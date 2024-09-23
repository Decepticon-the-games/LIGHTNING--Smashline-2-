use super::*;

unsafe extern "C" fn game_specialairs2end(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::SET_SPEED_EX(fighter, 1.4, -0.019, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 1.1, -0.519, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}

pub fn install() {
    Agent::new("miifighter")
        .game_acmd("game_specialairs2end", game_specialairs2end, Priority::Low)
        .install();
}
