use super::*;

unsafe extern "C" fn game_specialsjump(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        //enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 100, 23, 0, 3.0, 0.0, 0.0, 10.0, Some(0.0), Some(0.0), Some(2.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(fighter.module_accessor);
//whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("littlemac")
        .game_acmd("game_specialsjump", game_specialsjump, Priority::Low)
        .install();
}
