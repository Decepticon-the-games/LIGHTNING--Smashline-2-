use super::*;

unsafe extern "C" fn game_attackhi3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM, Hash40::new("attack_hi3"), false, -1.0);
    }
    macros::FT_MOTION_RATE(fighter, 0.833);
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("havel"), 8.0, 98, 76, 0, 52, 3.5, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("havel"), 8.0, 98, 76, 0, 52, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
    wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
    }
    macros::FT_MOTION_RATE(fighter, 0.842);
    frame(fighter.lua_state_agent, 54.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}    

pub fn install() {
    Agent::new("shizue")
        .game_acmd("game_attackhi3", game_attackhi3, Priority::Low)
        .install();
}
