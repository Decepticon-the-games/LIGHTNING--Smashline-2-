use super::*;

unsafe extern "C" fn game_attackairf(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_JUMP_MINI_FRAME) == 0 {
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.5);
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.5);
        }
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            if macros::is_excute(fighter) {
                enable_attack_cancel(fighter); 
                macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 4.5, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 4.5, 45, 134, 0, 27, 2.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 4.5, 45, 134, 0, 27, 2.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 4.5, 45, 134, 0, 27, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 4.5, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 5, 0, Hash40::new("haver"), 4.5, 57, 134, 0, 20, 2.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 6, 0, Hash40::new("haver"), 4.5, 57, 134, 0, 20, 2.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 7, 0, Hash40::new("top"), 4.5, 57, 134, 0, 20, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 6, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 7, -3.0, false);
                WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            if macros::is_excute(fighter) {
                enable_attack_cancel(fighter); 
                macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 3.4, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 5, 0, Hash40::new("haver"), 3.4, 57, 134, 0, 20, 2.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 6, 0, Hash40::new("haver"), 3.4, 57, 134, 0, 20, 2.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 7, 0, Hash40::new("top"), 3.4, 57, 134, 0, 20, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 6, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 7, -3.0, false);
                WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            if macros::is_excute(fighter) {
                enable_attack_cancel(fighter); 
                macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 4.0, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 4.0, 45, 134, 0, 27, 2.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 4.0, 45, 134, 0, 27, 2.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 45, 134, 0, 27, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 4.0, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 5, 0, Hash40::new("haver"), 4.0, 57, 134, 0, 20, 2.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 6, 0, Hash40::new("haver"), 4.0, 57, 134, 0, 20, 2.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 7, 0, Hash40::new("top"), 4.0, 57, 134, 0, 20, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 6, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 7, -3.0, false);
                WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            if macros::is_excute(fighter) {
                enable_attack_cancel(fighter); 
                macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.7, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 3.7, 45, 134, 0, 27, 2.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.7, 45, 134, 0, 27, 2.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.7, 45, 134, 0, 27, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 3.7, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 5, 0, Hash40::new("haver"), 3.7, 57, 134, 0, 20, 2.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 6, 0, Hash40::new("haver"), 3.7, 57, 134, 0, 20, 2.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 7, 0, Hash40::new("top"), 3.7, 57, 134, 0, 20, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 6, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 7, -3.0, false);
                WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            if macros::is_excute(fighter) {
                enable_attack_cancel(fighter); 
                macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.4, 45, 134, 0, 27, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 3.4, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 5, 0, Hash40::new("haver"), 3.4, 57, 134, 0, 20, 2.3, 0.0, 3.6, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 6, 0, Hash40::new("haver"), 3.4, 57, 134, 0, 20, 2.3, 0.0, 7.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 7, 0, Hash40::new("top"), 3.4, 57, 134, 0, 20, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, -5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 5, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 6, -3.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 7, -3.0, false);
                WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
        }
        else {
            if macros::is_excute(fighter) {
                enable_attack_cancel(fighter); 
                macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.72, 60, 50, 0, 72, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 2.72, 60, 50, 0, 72, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 0, 1, 2, 3, 0.25);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter, 4, 5, 6, 7, 0.25);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, -10.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, -10.0, false);
            }
        }
    }
    else {
        frame(fighter.lua_state_agent, 0.0);
        if macros::is_excute(fighter) {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 8.0);
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND {
            if macros::is_excute(fighter) {
                enable_attack_cancel(fighter); 
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 14.175, 361, 63, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 14.175, 361, 63, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 14.175, 361, 69, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 14.175, 361, 69, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                WorkModule::set_float(fighter.module_accessor, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
            wait(fighter.lua_state_agent, 1.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 14.175, 275, 67, 0, 25, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
            wait(fighter.lua_state_agent, 1.0);
            AttackModule::clear(fighter.module_accessor, 2, false);
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 16.2, 361, 63, 0, 48, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 16.2, 361, 63, 0, 48, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 16.2, 275, 58, 0, 25, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 16.2, 361, 67, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
            if macros::is_excute(fighter) {
                enable_attack_cancel(fighter); 
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.5, 361, 81, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.5, 361, 81, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 10.5, 361, 81, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 10.5, 361, 81, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                WorkModule::set_float(fighter.module_accessor, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
            wait(fighter.lua_state_agent, 1.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 10.5, 275, 67, 0, 25, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
            wait(fighter.lua_state_agent, 1.0);
            AttackModule::clear(fighter.module_accessor, 2, false);
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.0, 361, 63, 0, 48, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 361, 63, 0, 48, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 12.0, 275, 58, 0, 25, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 12.0, 361, 67, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_IRON {
            if macros::is_excute(fighter) {
                enable_attack_cancel(fighter); 
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.6, 361, 63, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.6, 361, 63, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 12.6, 361, 69, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 12.6, 361, 69, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                WorkModule::set_float(fighter.module_accessor, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
            wait(fighter.lua_state_agent, 1.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 12.6, 275, 58, 0, 25, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
            wait(fighter.lua_state_agent, 1.0);
            AttackModule::clear(fighter.module_accessor, 2, false);
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 14.4, 361, 63, 0, 48, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 14.4, 361, 63, 0, 48, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 14.4, 275, 58, 0, 25, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 14.4, 361, 67, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_STONE {
            if macros::is_excute(fighter) {
                enable_attack_cancel(fighter); 
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 11.55, 361, 63, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 11.55, 361, 63, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 11.55, 361, 69, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 11.55, 361, 69, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                WorkModule::set_float(fighter.module_accessor, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
            wait(fighter.lua_state_agent, 1.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 11.55, 275, 58, 0, 25, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
            wait(fighter.lua_state_agent, 1.0);
                AttackModule::clear(fighter.module_accessor, 2, false);
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 13.2, 361, 63, 0, 48, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.2, 361, 63, 0, 48, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 13.2, 275, 58, 0, 25, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 13.2, 361, 67, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_WOOD {
            if macros::is_excute(fighter) {
                enable_attack_cancel(fighter); 
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.5, 361, 63, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.5, 361, 63, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 10.5, 361, 69, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 10.5, 361, 69, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                WorkModule::set_float(fighter.module_accessor, 6.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            }
            wait(fighter.lua_state_agent, 1.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 10.5, 275, 58, 0, 25, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
            wait(fighter.lua_state_agent, 1.0);
            if macros::is_excute(fighter) {
                AttackModule::clear(fighter.module_accessor, 2, false);
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.0, 361, 63, 0, 48, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 361, 63, 0, 48, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 12.0, 275, 58, 0, 25, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 12.0, 361, 67, 0, 56, 4.2, 0.0, 4.8, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
        else{
            if macros::is_excute(fighter) {
                enable_attack_cancel(fighter); 
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.4, 361, 63, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.4, 361, 63, 0, 56, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
            wait(fighter.lua_state_agent, 1.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.6, 361, 63, 0, 48, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.6, 361, 63, 0, 48, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            whiff_cancel(fighter);
        }
    }
    frame(fighter.lua_state_agent, 14.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND) == *FIGHTER_PICKEL_MATERIAL_KIND_GOLD {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 2.25);
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.25);
        }
    }
    else{
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 2.0);
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
        }
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }    
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
}

pub fn install() {
    Agent::new("pickel")
        .game_acmd("game_attackairf", game_attackairf, Priority::Low)
        .install();
}
