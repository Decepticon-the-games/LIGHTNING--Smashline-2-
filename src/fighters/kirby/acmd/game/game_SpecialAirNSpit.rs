use super::*;

unsafe extern "C" fn game_specialairnspit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;


    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 361, 0, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_SPIT);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STARMISSILE, false, -1);
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_N_WORK_INT_TARGET_TASK);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_N_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_N_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("kirby")
        .game_acmd("game_specialairnspit", game_specialairnspit, Priority::Low)
        .install();
}
