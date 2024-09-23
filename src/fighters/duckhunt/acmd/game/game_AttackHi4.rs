use super::*;

unsafe extern "C" fn game_attackhi4(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, 5, *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_DELAY_FRAME);
        WorkModule::set_int(fighter.module_accessor, 6, *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_RETICLE_DISPLAY_FRAME);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 368, 100, 0, 0, 5.3, 0.0, 11.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), &Vector2f{x: -7.0, y: 10.0}, 8, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 368, 100, 0, 0, 6.3, 0.0, 15.0, -8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 17.0}, 8, false);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 85, 120, 0, 45, 7.8, 0.0, 21.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter); 
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter); 
    }
}    

pub fn install() {
    Agent::new("duckhunt")
        .game_acmd("game_attackhi4", game_attackhi4, Priority::Low)
        .install();
}
