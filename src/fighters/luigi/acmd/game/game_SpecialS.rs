use super::*;

unsafe extern "C" fn game_specials(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::SA_SET(fighter, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_RAM);
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_AIR);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 10.0, 361, 80, 0, 30, 4.8, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
        JostleModule::set_status(fighter.module_accessor, false);
        enable_attack_cancel(fighter); 
    }

    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 10.0, 361, 80, 0, 30, 4.8, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        enable_attack_cancel(fighter); 
    }

    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
    }
}    

pub fn install() {
    Agent::new("luigi")
        .game_acmd("game_specials", game_specials, Priority::Low)
        .install();
}
