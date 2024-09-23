use super::*;
pub static mut UPTAUNT_CANCELL : [bool; 8] = [false; 8]; 

//#[fighter_frame( agent = FIGHTER_KIND_CAPTAIN )]

unsafe extern "C" fn captain_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let status_kind = StatusModule::status_kind(fighter.module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            
            
            



//Side special cancel taunt
            
            if UPTAUNT_CANCELL[entry_id] {
                
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
                || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
                || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
                UPTAUNT_CANCELL[entry_id] = false;
                
            }   
        }
    }

pub fn install() {
    Agent::new("captain")
    .on_line(Main, captain_opff)
    .install();
}