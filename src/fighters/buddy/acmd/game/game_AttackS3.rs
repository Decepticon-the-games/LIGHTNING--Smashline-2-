use super::*;

unsafe extern "C" fn game_attacks3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, Hash40::new("attack_s3_s"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.75);
    frame(fighter.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter); 
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 46, 2, 0, 42, 2.6, 0.0, 7.2, 18.4, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 46, 2, 0, 42, 3.2, 0.0, 7.2, 13.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 46, 2, 0, 42, 3.6, 0.0, 7.2, 7.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0, 7.2, 20.2);
        attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0, 7.2, 15.6);
        attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0, 7.2, 9.6);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}    

pub fn install() {
    Agent::new("buddy")
        .game_acmd("game_attacks3", game_attacks3, Priority::Low)
        .install();
}
