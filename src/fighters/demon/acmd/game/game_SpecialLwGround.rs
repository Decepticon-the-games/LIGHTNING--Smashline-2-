use super::*;

unsafe extern "C" fn game_speciallwground(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 17.0, 60, 60, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 80, 4.0, 0.0, 4.0, -7.0, Some(0.0), Some(4.0), Some(11.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::FT_CATCH_STOP(fighter, 10, 1);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 15, 3);
        set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
        set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: -2.0, z: 0.0});
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_HIT);
        macros::CAM_ZOOM_OUT(fighter);
    }
    frame(fighter.lua_state_agent, 35.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 38.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 39.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 40.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 41.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 42.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}    

pub fn install() {
    Agent::new("demon")
        .game_acmd("game_speciallwground", game_speciallwground, Priority::Low)
        .install();
}
