use super::*;

unsafe extern "C" fn game_specialn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;    

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.1);
    frame(fighter.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("throw"), 4.0, -0.9, -0.5, 0.0,None, None,None,  *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(fighter, 1, Hash40::new("throw"), 3.5, -2.0, -0.5, 0.0,None, None,None,  *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 2, Hash40::new("top"), 5.9, 0.0, 7.0, 9.0,None, None,None,  *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
        }
        else{
        if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("mouth2"), 4.0, -0.9, -0.5, 0.0,None, None,None,  *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(fighter, 1, Hash40::new("mouth2"), 3.5, -2.0, -0.5, 0.0, None, None,None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 2, Hash40::new("top"), 5.9, 0.0, 7.0, 9.0,None, None,None,  *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
        }
    }
}
if macros::is_excute(fighter) {
    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 1.0, 0, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
}
frame(fighter.lua_state_agent, 18.0);
if macros::is_excute(fighter) {
    grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
}
frame(fighter.lua_state_agent, 21.0);
if macros::is_excute(fighter) {
    grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    AttackModule::clear_all(fighter.module_accessor);
    GrabModule::set_rebound(fighter.module_accessor, false);
    whiff_cancel(fighter);
}
}    

pub fn install() {
    Agent::new("yoshi")
        .game_acmd("game_specialn", game_specialn, Priority::Low)
        .install();
}
