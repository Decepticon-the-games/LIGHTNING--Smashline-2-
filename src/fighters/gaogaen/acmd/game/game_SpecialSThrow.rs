use super::*;

unsafe extern "C" fn game_specialsthrow(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
    }
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.75);
    frame(fighter.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_THROW_ROPE);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 60.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 95.0);
}    

pub fn install() {
    Agent::new("gaogaen")
        .game_acmd("game_specialsthrow", game_specialsthrow, Priority::Low)
        .install();
}
