
unsafe extern "C" fn game_landingairn(fighter: &mut L2CAgentBase) {
    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW) {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, Hash40::new("landing_air_n"), false, -1.0);
        }
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter); 
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 30, 120, 0, 40, 4.0, 0.0, 6.7, -5.0, Some(0.0), Some(6.7), Some(6.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel_cancel(fighter); 
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}pub fn install() {
    Agent::new("master")
        .game_acmd("game_landingairn", game_landingairn, Priority::Low)
        .install();
}
