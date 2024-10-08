use super::*;

unsafe extern "C" fn game_specialsdragf(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 2.0, 100, 100, 100, 50, 6.5, 0.0, 1.0, 2.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
}    

pub fn install() {
    Agent::new("ridley")
        .game_acmd("game_specialsdragf", game_specialsdragf, Priority::Low)
        .install();
}
