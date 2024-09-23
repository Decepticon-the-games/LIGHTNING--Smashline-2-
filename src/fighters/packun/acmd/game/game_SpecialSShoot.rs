use super::*;

unsafe extern "C" fn game_specialsshoot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.75);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 10.0, 3.0);
    }
    frame(fighter.lua_state_agent, 2.0);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 18, 100, 30, 0, 5.0, 0.0, 7.0, 7.0, Some(0.0), Some(7.0), Some(10.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PACKUN_GENERATE_ARTICLE_POISONBREATH, false, -1);
            whiff_cancel(fighter);
        }
    }
    frame(fighter.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(fighter, 0.9);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 5.0, 5.0);
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_CHANGE_KINETIC);
    }
    frame(fighter.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(fighter, 0.55);
}    

pub fn install() {
    Agent::new("packun")
        .game_acmd("game_specialsshoot", game_specialsshoot, Priority::Low)
        .install();
}
