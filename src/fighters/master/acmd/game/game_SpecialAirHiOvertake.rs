use super::*;

unsafe extern "C" fn game_specialairhiovertake(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, Hash40::new("special_air_hi_overtake"), false, -1.0);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.5, 260, 40, 0, 85, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 1, 6.5, 114, 65, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, 2, 6.5, 260, 110, 0, 14, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 9.0);
    
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_HI_FLAG_TARGET_AIR) {
        

        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_HI_FLAG_TARGET_HI_DAMAGE) {
            if macros::is_excute(fighter) {
                macros::CHECK_FINISH_CAMERA(fighter, -1, -9);
                lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.9);
                lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: -3.0, y: -6.0, z: 0.0});
            }
        }
        else{
            if macros::is_excute(fighter) {
                macros::CHECK_FINISH_CAMERA(fighter, -1, -9);
                lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.9);
                lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: -3.0, y: -6.0, z: 0.0});
            }
        }
    }
    frame(fighter.lua_state_agent, 11.0);
    
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_HI_FLAG_TARGET_AIR) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_HI_FLAG_TARGET_HI_DAMAGE) {
            if macros::is_excute(fighter) {
                let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
                let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
                let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
                macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, Hash40::new("throw"), target, target_group, target_no);
                whiff_cancel(fighter);
            }
        }            
        else{
            if macros::is_excute(fighter) {
                let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
                let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
                let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
                macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, Hash40::new("throw"), target, target_group, target_no);
                whiff_cancel(fighter);
            }
        }
    }
    else{
        if macros::is_excute(fighter) {
            let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
            whiff_cancel(fighter);
        }
    }
}

pub fn install() {
    Agent::new("master")
        .game_acmd("game_specialairhiovertake", game_specialairhiovertake, Priority::Low)
        .install();
}
