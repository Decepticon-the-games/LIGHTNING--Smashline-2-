use super::*;

unsafe extern "C" fn game_attackhi4(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_TRENCHMORTAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_TRENCHMORTAR, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_TRENCHMORTAR, Hash40::new("start"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 96, 100, 120, 0, 5.0, 0.0, 4.0, 1.0, Some(0.0), Some(6.0), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 96, 100, 120, 0, 5.0, 0.0, 4.0, 7.0, Some(0.0), Some(6.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_TRENCHMORTAR, Hash40::new("shoot"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 57.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_TRENCHMORTAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }  
}  

pub fn install() {
    Agent::new("snake")
        .game_acmd("game_attackhi4", game_attackhi4, Priority::Low)
        .install();
}
