use super::*;

unsafe extern "C" fn game_specials(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_ATTACK_TRIGGER);
        macros::FT_ADD_DAMAGE(fighter, 1.5);
        enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 90, 0, 15, 3.2, 0.0, 3.3, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_CALC_ATTACK_POWER);
        AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_BRAKE_TRIGGER);
    }
}

pub fn install() {
    Agent::new("pichu")
        .game_acmd("game_specials", game_specials, Priority::Low)
        .install();
}
