use super::*;

unsafe extern "C" fn game_specialairlwfallend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.7, z: 0.0});
        whiff_cancel(fighter);
    }
    wait(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_END_CHANGE_KINETIC);
    }
}    

pub fn install() {
    Agent::new("packun")
        .game_acmd("game_specialairlwfallend", game_specialairlwfallend, Priority::Low)
        .install();
}