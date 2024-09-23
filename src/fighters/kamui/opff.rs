use super::*;

//#[fighter_frame( agent = FIGHTER_KIND_KAMUI )]

unsafe extern "C" fn kamui_opff(fighter : &mut L2CFighterCommon) {
        unsafe {

    }
}

#[weapon_frame( agent = WEAPON_KIND_KAMUI_SPEARHAND )]
unsafe extern "C" fn kamui_spearhand(weapon : &mut L2CFighterBase) {
    unsafe {
        let entry_id = WorkModule::get_int(weapon.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;   

        if ENABLE_ATTACK_CANCEL[entry_id] {
            if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_ALL) {
                ATTACK_CANCEL[entry_id] = true; 
                cancel_on_hit(weapon);
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
            else if CANCEL_IN_NEUTRAL[entry_id] {
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
        }
    }
}
#[weapon_frame( agent = WEAPON_KIND_KAMUI_DRAGONHAND )]
unsafe extern "C" fn kamui_dragonhand(weapon : &mut L2CFighterBase) {
    unsafe {
        let entry_id = WorkModule::get_int(weapon.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;   

        if ENABLE_ATTACK_CANCEL[entry_id] {
            if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_ALL) {
                ATTACK_CANCEL[entry_id] = true; 
                cancel_on_hit(weapon);
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
            else if CANCEL_IN_NEUTRAL[entry_id] {
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
        }
    }
}
#[weapon_frame( agent = WEAPON_KIND_KAMUI_WATERDRAGON )]
unsafe extern "C" fn kamui_watergragon(weapon : &mut L2CFighterBase) {
    unsafe {
        let entry_id = WorkModule::get_int(weapon.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;   

        if ENABLE_ATTACK_CANCEL[entry_id] {
            if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_ALL) {
                ATTACK_CANCEL[entry_id] = true; 
                cancel_on_hit(weapon);
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
            else if CANCEL_IN_NEUTRAL[entry_id] {
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
        }
    }
}
pub fn install() {
    Agent::new("mario")
    .on_line(Main, mario_frame)
    .install();(
        kamui_spearhand, 
        kamui_dragonhand,
        kamui_watergragon
    );

}