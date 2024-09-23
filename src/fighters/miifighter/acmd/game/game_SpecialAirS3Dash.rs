use super::*;

unsafe extern "C" fn game_specialairs3dash(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 4.5, 4.5, 6.0, 6.0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 350, 100, 100, 0, 5.0, 0.0, 4.0, 0.0, Some(0.0), Some(4.0), Some(1.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, true, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 350, 100, 40, 0, 3.0, 0.0, 4.0, 3.0, Some(0.0), Some(9.0), Some(3.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 350, 100, 80, 0, 5.0, 0.0, 4.0, 1.0, Some(0.0), Some(4.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_STAGE, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 350, 100, 15, 0, 3.0, 0.0, 7.0, 3.0, Some(0.0), Some(8.0), Some(3.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_STAGE, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 6.0, 3.5, Some(0.0), Some(6.0), Some(2.0), *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_AIR_CAPTURED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.0, 0.0, 7.0, 5.5, Some(0.0), Some(8.0), Some(5.5), *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_AIR_CAPTURED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        whiff_cancel(fighter);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_SUPLEX_FLAG_REQUEST_GRAVITY);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 4.0, 3.0, 7.0, 7.0);
    }
    frame(fighter.lua_state_agent, 32.0);
    macros::FT_MOTION_RATE(fighter, 0.6);
}    

pub fn install() {
    Agent::new("miifighter")
        .game_acmd("game_specialairs3dash", game_specialairs3dash, Priority::Low)
        .install();
}
