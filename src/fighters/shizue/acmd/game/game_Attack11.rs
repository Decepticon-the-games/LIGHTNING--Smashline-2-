use super::*;

unsafe extern "C" fn game_attack11(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER, false, -1);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter);  
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 30, 0, 14, 2.5, 0.0, 5.5, 5.0, None, None, None, 0.8, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SHIZUE_HAMMER, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 30, 0, 14, 2.5, 0.0, 5.5, 8.0, None, None, None, 0.8, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SHIZUE_HAMMER, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 30, 0, 14, 2.5, 0.0, 5.5, 11.0, None, None, None, 0.8, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SHIZUE_HAMMER, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 14.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 14.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 14.0, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}    

pub fn install() {
    Agent::new("shizue")
        .game_acmd("game_attack11", game_attack11, Priority::Low)
        .install();
}
