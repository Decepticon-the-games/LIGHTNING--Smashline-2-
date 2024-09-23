use super::*;

unsafe extern "C" fn bayonetta_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
            
        let frame = MotionModule::frame(fighter.module_accessor);
        
        
        
        let fair_combo_flag =  WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO);
        if SearchModule::is_inflict(fighter.module_accessor) && SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_NORMAL) == 0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
        if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP {//gives her her normal second up b back
            WorkModule::set_int(fighter.module_accessor, 0, DEADFALL);
        }
    }
}

pub fn install() {
    Agent::new("bayonetta")
        .on_line(Main, bayonetta_opff)
        .install();
}
  