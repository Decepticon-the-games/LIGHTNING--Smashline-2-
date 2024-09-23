use super::*;

unsafe extern "C" fn game_specials2nana_nana(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
    }
}

pub fn install() {
    Agent::new("popo")
        .game_acmd("game_specials2nana_nana", game_specials2nana_nana, Priority::Low)
        .install();
}
