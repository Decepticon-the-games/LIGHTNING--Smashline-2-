use super::*;

unsafe extern "C" fn game_specialairhichargef(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, DEADFALL);

    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("wingr2"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("wingl2"), *HIT_STATUS_XLU);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 40, 66, 0, 75, 8.0, 0.0, 5.5, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("ridley")
        .game_acmd("game_specialairhichargef", game_specialairhichargef, Priority::Low)
        .install();
}
