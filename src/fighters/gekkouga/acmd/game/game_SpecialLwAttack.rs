use super::*;

unsafe extern "C" fn game_speciallwattack(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 5.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_LW_FLAG_ATTACK_UP) {
        if macros::is_excute(fighter) {
            enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 13.0, 90, 95, 0, 50, 8.5, 10.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 13.0, 90, 95, 0, 50, 7.0, 2.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
            AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        }
        else{
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_LW_FLAG_ATTACK_UP_SIDE) {
            if macros::is_excute(fighter) {
                enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 12.0, 60, 95, 0, 62, 7.5, 10.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 12.0, 60, 95, 0, 62, 6.0, 2.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
                AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
            }
            else{
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_LW_FLAG_ATTACK_SIDE) {
                if macros::is_excute(fighter) {
                    enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 11.0, 361, 85, 0, 62, 6.0, 10.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                    macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 11.0, 361, 85, 0, 62, 5.0, 2.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                    AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
                    AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
                }
                else{
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_LW_FLAG_ATTACK_DOWN_SIDE) {
                    if macros::is_excute(fighter) {
                        enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 12.0, 25, 72, 0, 62, 7.5, 10.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                        macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 12.0, 25, 72, 0, 62, 6.0, 2.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
                        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
                    }
                    else{
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_LW_FLAG_ATTACK_DOWN) {
                        if macros::is_excute(fighter) {
                            enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 13.0, 270, 65, 0, 25, 8.5, 10.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                            macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 13.0, 45, 95, 0, 50, 8.5, 10.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                            AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
                            AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
                        }
                    }
                }
            }
        }
    }
}    

}
}
}
frame(fighter.lua_state_agent, 20.0);
if macros::is_excute(fighter) {
AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
}
}

pub fn install() {
    Agent::new("gekkouga")
        .game_acmd("game_speciallwattack", game_speciallwattack, Priority::Low)
        .install();
}
