use super::*;

unsafe extern "C" fn game_attack13(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        //enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 2.0, 361, 10, 0, 35, 2.0, 0.0, 14.5, 10.0, None, None, None, 0.5, 5.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 2.0, 361, 10, 0, 35, 3.5, 0.0, 13.5, 9.0, None, None, None, 0.5, 5.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 1, Hash40::new("top"), 2.0, 361, 10, 0, 35, 3.5, 0.0, 13.5, 3.0, None, None, None, 0.5, 5.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 1, Hash40::new("top"), 2.0, 361, 10, 0, 35, 3.5, 0.0, 8.75, 9.0, Some(0.0), Some(13.5), Some(9.0), 0.5, 5.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 4, 1, Hash40::new("top"), 2.0, 361, 10, 0, 35, 3.5, 0.0, 9.0, 3.0, Some(0.0), Some(13.5), Some(3.0), 0.5, 5.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 5, 1, Hash40::new("top"), 2.0, 33, 15, 0, 35, 3.6, 0.0, 15.0, 3.0, Some(0.0), Some(15.0), Some(9.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}  

pub fn install() {
    Agent::new("demon")
        .game_acmd("game_attack13", game_attack13, Priority::Low)
        .install();
}
