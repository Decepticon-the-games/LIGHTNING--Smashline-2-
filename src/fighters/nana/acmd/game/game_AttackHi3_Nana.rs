use super::*;

unsafe extern "C" fn game_attackhi3_nana(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 8.0);
    for _ in 0..6 {
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 115, 10, 0, 65, 4.5, 2.0, 7.5, 0.0, Some(2.0), Some(7.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.6, 125, 10, 0, 60, 6.0, 2.0, 16.0, 0.0, Some(2.0), Some(16.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.6, 160, 10, 0, 20, 6.0, 2.0, 16.0, 0.0, Some(2.0), Some(16.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
                enable_multihit_cancel(fighter);
            }
            else{
                if macros::is_excute(fighter) {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 115, 10, 0, 65, 4.5, -2.0, 7.5, 0.0, Some(-2.0), Some(7.5), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
                    macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.6, 125, 10, 0, 60, 6.0, -2.0, 16.0, 0.0, Some(-2.0), Some(16.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
                    macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.6, 160, 10, 0, 20, 6.0, -2.0, 16.0, 0.0, Some(-2.0), Some(16.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
                    enable_multihit_cancel(fighter);
                }
            }
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 90, 118, 0, 38, 5.5, 2.0, 9.0, 0.0, Some(2.0), Some(9.0), Some(0.0), 2.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 90, 118, 0, 38, 7.0, 2.0, 17.0, 0.0, Some(2.0), Some(17.0), Some(0.0), 2.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
            enable_attack_cancel(fighter); 
        } 
        else{
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 90, 118, 0, 38, 5.5, -2.0, 9.0, 0.0, Some(-2.0), Some(9.0), Some(0.0), 2.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 90, 118, 0, 38, 7.0, -2.0, 17.0, 0.0, Some(-2.0), Some(17.0), Some(0.0), 2.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
                enable_attack_cancel(fighter); 
            } 
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
    AttackModule::clear_all(fighter.module_accessor);
    AttackModule::clear_all(fighter.module_accessor);
    whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("nana")
        .game_acmd("game_attackhi3_nana", game_attackhi3_nana, Priority::Low)
        .install();
}
