use super::*;

unsafe extern "C" fn game_attack100sub(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter); 
        macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 0.6, 361, 15, 0, 8, 2.25, 7.5, 0.8, -0.5, Some(2.0), Some(-1.0), Some(0.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.6, 361, 15, 0, 8, 6.5, 0.0, 8.0, 15.0, Some(0.0), Some(8.0), Some(12.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 9);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 9);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}    

pub fn install() {
    Agent::new("captain")
        .game_acmd("game_attack100sub", game_attack100sub, Priority::Low)
        .install();
}
