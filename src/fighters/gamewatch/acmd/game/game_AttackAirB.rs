use super::*;

unsafe extern "C" fn game_attackairb(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_TURTLE, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_air_b"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    for _ in 0..3 {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 100, 0, 0, 4.2, 0.0, 3.5, -14.3, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 367, 100, 0, 0, 3.3, 0.0, 1.8, -8.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}    
if macros::is_excute(fighter) {
    enable_attack_cancel(fighter); 
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 85, 0, 35, 6.0, 0.0, 3.5, -14.3, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 85, 0, 35, 4.5, 0.0, 1.8, -8.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
}
wait(fighter.lua_state_agent, 1.0);
if macros::is_excute(fighter) {
    AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
}
frame(fighter.lua_state_agent, 38.0);
if macros::is_excute(fighter) {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
}
frame(fighter.lua_state_agent, 38.0);
if macros::is_excute(fighter) {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
}
}    

pub fn install() {
    Agent::new("gamewatch")
        .game_acmd("game_attackairb", game_attackairb, Priority::Low)
        .install();
}
