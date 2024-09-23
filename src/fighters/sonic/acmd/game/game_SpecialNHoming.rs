use super::*;

unsafe extern "C" fn game_specialnhoming(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        MotionModule::set_rate(fighter.module_accessor, 80);
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_N_HOMING_FLAG_IS_KIRBY) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 7.0, 53, 65, 0, 55, 5.0, 0.5, 1.5, 0.0, Some(0.5), Some(1.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
            AttackModule::set_captured_same_time_attack(fighter.module_accessor, 0, true);
        }
        else{
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 7.0, 53, 65, 0, 55, 5.0, 0.5, 1.5, 0.0, Some(0.5), Some(1.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
            AttackModule::set_captured_same_time_attack(fighter.module_accessor, 0, true);
        }
    }
}    

pub fn install() {
    Agent::new("sonic")
        .game_acmd("game_specialnhoming", game_specialnhoming, Priority::Low)
        .install();
}
