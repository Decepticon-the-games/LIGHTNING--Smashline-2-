use super::*;

unsafe extern "C" fn game_specialairlwshoot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_OIL, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_OIL, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        enable_attack_cancel(fighter); 
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 80, 0, 25, 3.4, 0.0, 7.7, 11.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        //AttackModule::set_special_paint(fighter.module_accessor, 0, *SPECIAL_PAINT_OIL);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 78, 0, 25, 4.5, 0.0, 9.5, 20.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        //AttackModule::set_special_paint(fighter.module_accessor, 0, *SPECIAL_PAINT_OIL);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 76, 0, 25, 8.0, 0.0, 7.0, 32.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        //AttackModule::set_special_paint(fighter.module_accessor, 0, *SPECIAL_PAINT_OIL);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("gamewatch")
        .game_acmd("game_specialairlwshoot", game_specialairlwshoot, Priority::Low)
        .install();
}
