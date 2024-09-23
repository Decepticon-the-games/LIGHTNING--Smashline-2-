use super::*;

unsafe extern "C" fn game_specials1start(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING_MUL_SPEED_X);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_MOTION_SPEED_MUL);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 2.0, 3.0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 60, 50, 130, 0, 4.0, 0.0, 7.0, 3.8, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KNEE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_HIT_CHECK_ONOFF);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_HIT_CHECK_ONOFF);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 3.0);
    }
}

pub fn install() {
    Agent::new("miifighter")
        .game_acmd("game_specials1start", game_specials1start, Priority::Low)
        .install();
}
