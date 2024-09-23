use super::*;

unsafe extern "C" fn game_specialairsdash(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR);
        JostleModule::set_status(fighter.module_accessor, false);
        enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 22.0, 43, 64, 0, 66, 3.8, 0.0, 4.2, 1.8, Some(0.0), Some(4.2), Some(3.2), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 22.0, 43, 64, 0, 66, 4.2, 0.0, 9.2, 3.8, Some(0.0), Some(9.2), Some(5.4), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_captured_same_time_attack(fighter.module_accessor, 0, true);
        AttackModule::set_captured_same_time_attack(fighter.module_accessor, 1, true);
        AttackModule::set_captured_same_time_attack_damage_mul(fighter.module_accessor, 0, 0.25);
        AttackModule::set_captured_same_time_attack_damage_mul(fighter.module_accessor, 1, 0.25);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.48);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        macros::HIT_NO(fighter, 0, *HIT_STATUS_INVINCIBLE);
        macros::HIT_NO(fighter, 1, *HIT_STATUS_INVINCIBLE);
        macros::HIT_NO(fighter, 2, *HIT_STATUS_INVINCIBLE);
        macros::HIT_NO(fighter, 3, *HIT_STATUS_INVINCIBLE);
        macros::HIT_NO(fighter, 4, *HIT_STATUS_INVINCIBLE);
        macros::HIT_NO(fighter, 5, *HIT_STATUS_INVINCIBLE);
        macros::HIT_NO(fighter, 6, *HIT_STATUS_INVINCIBLE);
        macros::HIT_NO(fighter, 7, *HIT_STATUS_INVINCIBLE);
        macros::HIT_NO(fighter, 8, *HIT_STATUS_INVINCIBLE);
        macros::HIT_NO(fighter, 9, *HIT_STATUS_INVINCIBLE);
        macros::HIT_NO(fighter, 10, *HIT_STATUS_INVINCIBLE);
        macros::HIT_NO(fighter, 11, *HIT_STATUS_NORMAL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_CLIFF_CHECK);
    }
    wait(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 43, 57, 0, 66, 3.8, 0.0, 4.2, 1.8, Some(0.0), Some(4.2), Some(3.2), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 43, 57, 0, 66, 4.2, 0.0, 9.2, 3.8, Some(0.0), Some(9.2), Some(5.4), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_captured_same_time_attack(fighter.module_accessor, 0, true);
        AttackModule::set_captured_same_time_attack(fighter.module_accessor, 1, true);
        AttackModule::set_captured_same_time_attack_damage_mul(fighter.module_accessor, 0, 0.25);
        AttackModule::set_captured_same_time_attack_damage_mul(fighter.module_accessor, 1, 0.25);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.28);
    }
}    

pub fn install() {
    Agent::new("buddy")
        .game_acmd("game_specialairsdash", game_specialairsdash, Priority::Low)
        .install();
}
