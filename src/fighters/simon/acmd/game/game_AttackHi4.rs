use super::*;

unsafe extern "C" fn game_attackhi4(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.2);
    frame(fighter.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 3.0, 0.0, 18.0, 2.0, Some(0.0), Some(52.0), Some(2.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        //battle_object();
        //methodlib::L2CValue::L2CValue(void*)();
    }
    else{
    //methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
}
//methodlib::L2CValue::as_pointer()const(0, 1);
//set_whip_reflect_attack_off_id();
enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 90, 91, 0, 64, 3.0, 0.0, 52.0, 2.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 90, 85, 0, 63, 3.0, 0.0, 18.0, 2.0, Some(0.0), Some(52.0), Some(2.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
frame(fighter.lua_state_agent, 27.0);
if macros::is_excute(fighter) {
    AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
}
}    

pub fn install() {
    Agent::new("simon")
        .game_acmd("game_attackhi4", game_attackhi4, Priority::Low)
        .install();
}
