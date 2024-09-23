use super::*;
static mut UP_SPECIAL_HIT : [bool; 8] = [false; 8];
static mut UP_SPECIAL_HIT_COUNT : [i32; 8] = [0; 8];

//#[fighter_frame( agent = FIGHTER_KIND_SONIC )]

unsafe extern "C" fn sonic_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let status_kind = StatusModule::status_kind(fighter.module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            
            let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
            
        }
    
    }

pub fn install() {
    Agent::new("mario")
    .on_line(Main, mario_frame)
    .install();(sonic_opff);

}