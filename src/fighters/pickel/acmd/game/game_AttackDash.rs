use super::*;

unsafe extern "C" fn game_attackdash(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(fighter.lua_state_agent, 8.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
        if macros::is_excute(fighter) {
            enable_attack_cancel(fighter); 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.04, 50, 66, 0, 68, 5.8, 0.0, 8.8, 4.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.04, 50, 66, 0, 68, 5.8, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(fighter.module_accessor, 4.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.34, 50, 58, 0, 66, 5.2, 0.0, 8.8, 3.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.34, 50, 58, 0, 66, 5.2, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }        
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            enable_attack_cancel(fighter); 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.4, 50, 76, 0, 68, 5.8, 0.0, 8.8, 4.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.4, 50, 76, 0, 68, 5.8, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(fighter.module_accessor, 4.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.4, 50, 66, 0, 66, 5.2, 0.0, 8.8, 3.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.4, 50, 66, 0, 66, 5.2, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
        if macros::is_excute(fighter) {
            enable_attack_cancel(fighter); 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.48, 50, 66, 0, 68, 5.8, 0.0, 8.8, 4.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.48, 50, 66, 0, 68, 5.8, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(fighter.module_accessor, 4.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.08, 50, 58, 0, 66, 5.2, 0.0, 8.8, 3.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.08, 50, 58, 0, 66, 5.2, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
        if macros::is_excute(fighter) {
            enable_attack_cancel(fighter); 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.44, 50, 66, 0, 68, 5.8, 0.0, 8.8, 4.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.44, 50, 66, 0, 68, 5.8, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(fighter.module_accessor, 4.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.24, 50, 58, 0, 66, 5.2, 0.0, 8.8, 3.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.24, 50, 58, 0, 66, 5.2, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
        if macros::is_excute(fighter) {
            enable_attack_cancel(fighter); 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.4, 50, 66, 0, 68, 5.8, 0.0, 8.8, 4.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.4, 50, 66, 0, 68, 5.8, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(fighter.module_accessor, 4.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.4, 50, 58, 0, 66, 5.2, 0.0, 8.8, 3.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.4, 50, 58, 0, 66, 5.2, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
    else{
        if macros::is_excute(fighter) {
            enable_attack_cancel(fighter); 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.32, 50, 66, 0, 68, 5.8, 0.0, 8.8, 4.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.72, 50, 58, 0, 66, 5.2, 0.0, 8.8, 3.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.2);
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.2);
        }
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
}

pub fn install() {
    Agent::new("pickel")
        .game_acmd("game_attackdash", game_attackdash, Priority::Low)
        .install();
}
