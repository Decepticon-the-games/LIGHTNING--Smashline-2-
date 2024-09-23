use super::*;

unsafe extern "C" fn game_specialhistart(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_HI_FLAG_CHECK_COUPLE);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_HI_FLAG_CHECK_COUPLE);
    }
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 15.0);
    get_value_float(*FT_VAR_FLOAT_STICK_X_BACK);
    WorkModule::get_param_float(fighter.module_accessor, 0, Hash40::new("param_special_hi"), Hash40::new("start_turn_cont_x"));
    if(0x19e320(32)){
        if macros::is_excute(fighter) {
            macros::STICK_LR(fighter);
            macros::UPDATE_ROT(fighter);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_HI_FLAG_DETACH_PARTNER);
    }
}

pub fn install() {
    Agent::new("popo")
        .game_acmd("game_specialhistart", game_specialhistart, Priority::Low)
        .install();
}
