use super::*;

unsafe extern "C" fn game_specialairs1end(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 1, 1, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING_MUL_SPEED_X);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING_MUL_SPEED_X);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 100, 30, 30, 7.0, 0.0, 8.0, 6.3, Some(0.0), Some(5.5), Some(6.3), 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 367, 100, 30, 30, 7.0, 0.0, 8.0, 6.3, Some(0.0), Some(5.5), Some(6.3), 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0.3, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 100, 16, 0, 6.5, 0.0, 8.0, 6.0, Some(0.0), Some(5.5), Some(6.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0.3, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 100, 50, 0, 6.5, 0.0, 9.0, 9.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.3, y: 0.5, z: 0.0});
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 100, 50, 0, 6.5, 0.0, 9.0, 9.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: -0.3, y: 0.7, z: 0.0});
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 60, 70, 40, 30, 6.5, 0.0, 8.0, 8.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.85, y: 3.0, z: 0.0});
        enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 70, 135, 0, 54, 7.0, 0.0, 16.0, 9.5, Some(0.0), Some(10.0), Some(8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 55.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_CONTROL_X);
    }
}

pub fn install() {
    Agent::new("miifighter")
        .game_acmd("game_specialairs1end", game_specialairs1end, Priority::Low)
        .install();
}
