use super::*;

unsafe extern "C" fn game_attacks3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 2.5, 0.0, 10.0, 5.0, Some(0.0), Some(10.0), Some(34.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
    }
    frame(fighter.lua_state_agent, 14.0);
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
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 65, 0, 65, 2.5, 0.0, 10.0, 34.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 65, 0, 65, 2.5, 0.0, 10.0, 7.0, Some(0.0), Some(10.0), Some(34.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 65, 0, 65, 4.5, 0.0, 10.0, 7.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
wait(fighter.lua_state_agent, 2.0);
if macros::is_excute(fighter) {
    AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
}
frame(fighter.lua_state_agent, 27.0);
if macros::is_excute(fighter) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_HOLD);
}
}   

pub fn install() {
    Agent::new("simon")
        .game_acmd("game_attacks3", game_attacks3, Priority::Low)
        .install();
}
