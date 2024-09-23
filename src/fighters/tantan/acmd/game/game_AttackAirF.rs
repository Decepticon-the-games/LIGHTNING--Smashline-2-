use super::*;

unsafe extern "C" fn game_attackairf(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if (WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R) ==1){
        frame(fighter.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.35);
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("armr5"), *HIT_STATUS_XLU);
            enable_attack_cancel(fighter);  
macros::ATTACK(fighter, 0, 0, Hash40::new("armr5"), 12.0, 60, 95, 0, 50, 2.5, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 12.0, 60, 95, 0, 50, 5.0, 3.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
        frame(fighter.lua_state_agent, 24.0);
        if macros::is_excute(fighter) {
            HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    }
    else if(WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R) ==2){
        frame(fighter.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(fighter, 0.75);
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("armr5"), *HIT_STATUS_XLU);
            enable_attack_cancel(fighter);  
macros::ATTACK(fighter, 0, 0, Hash40::new("armr5"), 6.0, 38, 85, 0, 45, 2.5, 1.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 6.0, 38, 85, 0, 45, 3.5, 3.5, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        }
        frame(fighter.lua_state_agent, 24.0);
        if macros::is_excute(fighter) {
            HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    }
    else{
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("armr5"), *HIT_STATUS_XLU);
            enable_attack_cancel(fighter);  
macros::ATTACK(fighter, 0, 0, Hash40::new("armr5"), 8.5, 50, 95, 0, 40, 2.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 8.5, 50, 95, 0, 40, 4.5, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        frame(fighter.lua_state_agent, 24.0);
        if macros::is_excute(fighter) {
            HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
        }
    }
   


frame(fighter.lua_state_agent, 37.0);
if macros::is_excute(fighter) {
WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
}
}

pub fn install() {
    Agent::new("tantan")
        .game_acmd("game_attackairf", game_attackairf, Priority::Low)
        .install();
}
