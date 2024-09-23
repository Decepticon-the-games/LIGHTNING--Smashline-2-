use super::*;

unsafe extern "C" fn game_speciallwhit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;


    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 8.0, 88, 30, 0, 100, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 8.0, 88, 30, 0, 100, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 8.0, 88, 30, 0, 100, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 4, 0, Hash40::new("haver"), 8.0, 88, 30, 0, 100, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::clear(fighter.module_accessor, 0, false);
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_FLAG_MOVE_ATTACK);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
    }
    }

pub fn install() {
    Agent::new("master_axe")
        .game_acmd("game_speciallwhit", game_speciallwhit, Priority::Low)
        .install();
}
