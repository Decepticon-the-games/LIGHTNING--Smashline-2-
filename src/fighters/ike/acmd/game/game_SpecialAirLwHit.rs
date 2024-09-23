use super::*;


unsafe extern "C" fn game_specialairlwhit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        enable_counter_cancel(fighter);
    }

    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
       enable_attack_cancel(fighter);
 macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 48, 9.0, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
        if macros::is_excute(fighter) {
            AttackModule::set_optional_hit_sound(fighter.module_accessor, 0, Hash40::new("se_ike_criticalhit"));
        }
    }
    frame(fighter.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(fighter, 1.3);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
    }  
}  

pub fn install() {
    Agent::new("ike")
        .game_acmd("game_specialairlwhit", game_specialairlwhit, Priority::Low)
        .install();
}
