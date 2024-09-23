use super::*;


unsafe extern "C" fn game_specialairlwhit(fighter: &mut L2CAgentBase) {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if entry_id < 8 {
        enable_counter_cancel(fighter);
    }

    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
enable_attack_cancel(fighter); 
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 68, 0, 87, 12.0, 0.0, 6.0, 11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 80, 68, 0, 87, 12.0, 0.0, 6.0, -11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
    }
    
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
        if macros::is_excute(fighter) {
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 0, Hash40::new("se_kamui_criticalhit"));
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 1, Hash40::new("se_kamui_criticalhit"));
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 90, 66, 0, 85, 8.0, 0.0, 21.0, 11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 8.0, 90, 66, 0, 85, 8.0, 0.0, 21.0, -11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 3, true, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
        if macros::is_excute(fighter) {
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 2, Hash40::new("se_kamui_criticalhit"));
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 3, Hash40::new("se_kamui_criticalhit"));
        }
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
 whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("kamui_waterdragon")
        .game_acmd("game_specialairlwhit", game_specialairlwhit, Priority::Low)
        .install();
}
