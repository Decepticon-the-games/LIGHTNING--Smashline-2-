use super::*;

unsafe extern "C" fn game_speciallw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
    }
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 0.6);
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.5, 0.0, 7.75, 12.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 2, Hash40::new("top"), 3.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, 3, Hash40::new("top"), 2.0, 0.0, 7.75, 12.5, Some(0.0), Some(7.75), Some(9.5), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.5, 0.0, 19.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 2, Hash40::new("top"), 3.0, 0.0, 14.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, 3, Hash40::new("top"), 2.0, 0.0, 19.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
        GrabModule::set_constraint(fighter.module_accessor, 3, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0.1, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    macros::FT_MOTION_RATE(fighter, 0.9);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(fighter.module_accessor, false);
        enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 19.0, 4.5, Some(0.0), Some(23.0), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 12.0, 6.0, Some(0.0), Some(16.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 64, 50, 0, 75, 3.0, 0.0, 3.0, 4.0, Some(0.0), Some(3.0), Some(5.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(fighter.module_accessor, 2, true);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 19.5, 4.5, Some(0.0), Some(23.5), Some(3.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 12.0, 6.0, Some(0.0), Some(16.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 20.0, 4.5, Some(0.0), Some(24.0), Some(3.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 25.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}    

pub fn install() {
    Agent::new("demon")
        .game_acmd("game_speciallw", game_speciallw, Priority::Low)
        .install();
}
