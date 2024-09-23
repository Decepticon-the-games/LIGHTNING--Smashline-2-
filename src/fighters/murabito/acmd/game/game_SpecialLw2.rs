use super::*;

unsafe extern "C" fn game_speciallw2(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_WATER);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_WATER);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("murabito")
        .game_acmd("game_speciallw2", game_speciallw2, Priority::Low)
        .install();
}
