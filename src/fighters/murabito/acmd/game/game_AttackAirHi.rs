use super::*;

unsafe extern "C" fn game_attackairhi(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 1{
        if macros::is_excute(fighter) {
            enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 85, 100, 0, 40, 5.2, 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
        else{
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 2{
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 85, 100, 0, 40, 6.5, 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            }
            else{
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 85, 100, 0, 40, 7.5, 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            }
        }
    }
}    
}
wait(fighter.lua_state_agent, 2.0);
if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 1 {
if macros::is_excute(fighter) {
    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 85, 100, 0, 40, 3.8, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
}
else{
if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM) == 2 {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 85, 100, 0, 40, 4.5, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    else{
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 85, 100, 0, 40, 4.8, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
}    

}
}
}
frame(fighter.lua_state_agent, 22.0);
if macros::is_excute(fighter) {
AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
}
frame(fighter.lua_state_agent, 39.0);
if macros::is_excute(fighter) {
WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
}
}

pub fn install() {
    Agent::new("murabito")
        .game_acmd("game_attackairhi", game_attackairhi, Priority::Low)
        .install();
}
