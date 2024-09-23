use super::*;

unsafe extern "C" fn game_specialairlw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 0.0);
    if (0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.3, y: -1.0, z: 0.0});
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        }
    }
    else{
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 1.3, y: -1.5, z: 0.0});
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        }
    }
      
    frame(fighter.lua_state_agent, 1.0);
    if (0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: -0.3, z: 0.0});
        }
    }
    else{
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: -1.0, z: 0.0});
        }
    }    
    
    frame(fighter.lua_state_agent, 2.0);
    if (0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        }
    }
    else{
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: -0.5, z: 0.0});
        }
    }
    
    frame(fighter.lua_state_agent, 3.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if (0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 70, 0, 80, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        else{
            if (0x2508e0(*ATTACK_REGION_PUNCH, *FIGHTER_DOLLY_STRENGTH_W)){
                if macros::is_excute(fighter) {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 50, 65, 0, 80, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
                }
            }
        }
        else{
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 75, 0, 80, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
            }
        }
        else{
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 50, 70, 0, 80, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }    
    frame(fighter.lua_state_agent, 4.0);
    if (0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        }
    }
    else{
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }
    
    frame(fighter.lua_state_agent, 5.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if(0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 4.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    else{
        if(0x2508e0(*ATTACK_REGION_PUNCH, *FIGHTER_DOLLY_STRENGTH_W)){
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 4.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    else{
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 310, 95, 0, 30, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
        }
    }
    else{
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 4.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    if (0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.2);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        }
    }
    else{
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }

    frame(fighter.lua_state_agent, 6.0);
    if (0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        }
    }
    else{
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }

    frame(fighter.lua_state_agent, 7.0);
    if (0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
            if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        }
    }
    else{
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }
    frame(fighter.lua_state_agent, 8.0);
    if (0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        }
    }
    else{
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }
    
    frame(fighter.lua_state_agent, 9.0);
    if (0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
    }
    }
    else{
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }
    
    frame(fighter.lua_state_agent, 10.0);
    if (0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        }
    }
    else{
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }
    
    frame(fighter.lua_state_agent, 11.0);
    if (0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        }
    }
    else{
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }
    
    frame(fighter.lua_state_agent, 12.0);
    if (0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        
        }
    }
    else{
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }
    frame(fighter.lua_state_agent, 13.0);
    if (0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        }
    }
    else{
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }
    
    frame(fighter.lua_state_agent, 14.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
    0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 4.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if (0x2508e0(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH, *FIGHTER_DOLLY_STRENGTH_W)){
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 10);
        }
    }
    else{
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 10);
        }
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_DOLLY_SPECIAL_LW_FALL);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_LANDING_HEAVY);
    }
}

pub fn install() {
    Agent::new("dolly")
        .game_acmd("game_specialairlw", game_specialairlw, Priority::Low)
        .install();
}
