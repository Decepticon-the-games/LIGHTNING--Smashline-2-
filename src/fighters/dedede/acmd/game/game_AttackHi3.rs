use super::*;

unsafe extern "C" fn game_attackhi3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 7.0, 6.0);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 11.5, 96, 24, 0, 65, 6.0, -1.5, 1.0, 1.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("head"), 9.5, 96, 24, 0, 65, 6.5, 4.8, 0.9, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 2.0, 6.0);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(fighter.lua_state_agent, 8.0);
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("dedede")
        .game_acmd("game_attackhi3", game_attackhi3, Priority::Low)
        .install();
}
