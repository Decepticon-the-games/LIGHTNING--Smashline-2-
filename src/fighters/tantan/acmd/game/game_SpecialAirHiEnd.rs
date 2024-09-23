use super::*;

unsafe extern "C" fn game_specialairhiend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, DEADFALL);

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 3, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 9, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 10, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 11, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 12, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 13, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 19, *HIT_STATUS_NORMAL);
    }
    frame(fighter.lua_state_agent, 2.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TANTAN_STATUS_SPECIAL_HI_FLAG_IS_BLOCKED) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L) {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 10.35, 45, 85, 0, 66, 5.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            }
            frame(fighter.lua_state_agent, 3.0);
            //methodlib::L2CValue::operatorbool()const(is_excute);
        }
        else{
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 9.0, 45, 78, 0, 56, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        }
        frame(fighter.lua_state_agent, 3.0);
        //methodlib::L2CValue::operatorbool()const(is_excute);
    }   
    AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

pub fn install() {
    Agent::new("tantan")
        .game_acmd("game_specialairhiend", game_specialairhiend, Priority::Low)
        .install();
}
