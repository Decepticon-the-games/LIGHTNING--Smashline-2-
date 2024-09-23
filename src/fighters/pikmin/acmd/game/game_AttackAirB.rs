use super::*;

unsafe extern "C" fn game_attackairb(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

frame(fighter.lua_state_agent, 9.0);
if macros::is_excute(fighter) {
    PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true; 
    macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 15.12, 40, 78, 0, 30, 3.0, 0.0, 3.5, 0.0, Some(0.0), Some(-3.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
    macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
}
frame(fighter.lua_state_agent, 13.0);
if macros::is_excute(fighter) {
    AttackModule::clear_all(fighter.module_accessor);
    whiff_cancel(fighter);
    HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
}
} 


unsafe extern "C" fn game_attackairb_b(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true; 
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 10.8, 40, 98, 0, 30, 3.0, 0.0, 3.5, 0.0, Some(0.0), Some(-3.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}   


unsafe extern "C" fn game_attackairb_v(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true; 
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 17.28, 40, 78, 0, 30, 3.0, 0.0, 3.5, 0.0, Some(0.0), Some(-3.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true; 
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}


unsafe extern "C" fn game_attackairb_w(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true; 
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 8.64, 40, 98, 0, 30, 3.0, 0.0, 3.5, 0.0, Some(0.0), Some(-3.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}


unsafe extern "C" fn game_attackairb_y(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true; 
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 10.8, 40, 98, 0, 30, 4.5, 0.0, 3.5, 0.0, Some(0.0), Some(-3.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PIKMIN);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

pub fn install() {
    Agent::new("pikmin_pikmin")
        .game_acmd("game_attackairb_y", game_attackairb_y, Priority::Low)
        .install();
}
