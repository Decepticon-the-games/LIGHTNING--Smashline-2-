use super::*;

unsafe extern "C" fn game_specialnhit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: -0.7, y: 0.0, z: 0.0});
    }
    macros::FT_MOTION_RATE(fighter, 0.6);
}pub fn install() {
    Agent::new("sonic")
        .game_acmd("game_specialnhit", game_specialnhit, Priority::Low)
        .install();
}
