use super::*;

unsafe extern "C" fn game_specialhidownboundd(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_NO_LAST_ATTACK);
    if(methodlib::L2CValue::operator==(lib::L2CValueconst&)const(false, true)){
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    else{
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_CRITICAL);
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 10.0, 50, 74, 0, 90, 12.0, 1.0, -1.0, 0.0, None, None, None, 1.3, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
    }
}    

pub fn install() {
    Agent::new("lucas")
        .game_acmd("game_specialhidownboundd", game_specialhidownboundd, Priority::Low)
        .install();
}
