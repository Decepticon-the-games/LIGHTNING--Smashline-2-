use super::*;

unsafe extern "C" fn game_specialnend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TANTAN_KIRBY_STATUS_SPECIAL_N_FLAG_ATTACK_BLOCKED) {
        if macros::is_excute(fighter) {
            enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 12.0, 361, 87, 0, 40, 3.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(8.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
//whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_TANTAN_GENERATE_ARTICLE_SPIRALSIMPLE, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}    

pub fn install() {
    Agent::new("tantan")
        .game_acmd("game_specialnend", game_specialnend, Priority::Low)
        .install();
}
