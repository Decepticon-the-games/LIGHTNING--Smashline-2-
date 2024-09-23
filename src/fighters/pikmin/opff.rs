use super::*;

//#[fighter_frame( agent = FIGHTER_KIND_PIKMIN )]

unsafe extern "C" fn pikmin_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let status_kind = StatusModule::status_kind(fighter.module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            
            let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
            
            println!("olimar {}", PIKMIN_ATTACK_CANCEL[entry_id]);

            if PIKMIN_ATTACK_CANCEL[entry_id]  {
                CancelModule::enable_cancel(fighter.module_accessor);
                PIKMIN_ATTACK_CANCEL[entry_id] = false;
            }

//New subtititle for any other code, if not applicable just delete the lines
            
        }
    }

//#[weapon_frame( agent = WEAPON_KIND_PIKMIN_PIKMIN )]

unsafe extern "C" fn pikmin_pikmin_opff(fighter : &mut L2CFighterBase) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let status_kind = StatusModule::status_kind(fighter.module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            
            let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
            
            println!("pikmin {}", PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id]);

        if PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id]  {
            if (AttackModule::is_attack_occur(fighter.module_accessor) && SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) == 0) {
                PIKMIN_ATTACK_CANCEL[entry_id] = true;
                PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = false;
            }  
        }

//New subtititle for any other code, if not applicable just delete the lines

        }
    }

pub fn install() {
    Agent::new("mario")
    .on_line(Main, mario_frame)
    .install();(pikmin_opff, pikmin_pikmin_opff);

}