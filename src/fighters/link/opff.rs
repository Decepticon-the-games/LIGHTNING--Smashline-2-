use super::*;

//#[fighter_frame( agent = FIGHTER_KIND_LINK )]

unsafe extern "C" fn link_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let status_kind = StatusModule::status_kind(fighter.module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            
            
            
        /*
                static mut UPSMASH_CANCEL_COUNT : [bool; 8] = [false; 8];
                static mut UPSMASH_CANCEL_COUNTER : [i32; 8] = [0; 8];

            //Cancel Up smash up to 2 times         
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING)  {
                

                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
                    if AttackModule::is_attack_occur(fighter.module_accessor) {
                        //Counter 
                        if UPSMASH_CANCEL_COUNT[entry_id] == false {
                            UPSMASH_CANCEL_COUNTER[entry_id] +=1;
                            UPSMASH_CANCEL_COUNT[entry_id] = true; 
                        }
                    }
                    else {
                        UPSMASH_CANCEL_COUNT[entry_id] = false;
                    }
                    //Disable cancel
                    if UPSMASH_CANCEL_COUNTER[entry_id] >2 {//How many times you can cancel
                        UPSMASH_CANCEL_COUNTER[entry_id] = 3;//How  many hits before disabling cancel
                        //ENABLE_MULTIHIT_CANCEL[entry_id] = false; 
                    }
                    else {
                        enable_multihit_cancel(fighter); 
                    }

                    //Reset
                    if MOVEMENT_CANCEL[entry_id] {
                        if UPSMASH_CANCEL_COUNTER[entry_id] == 3 {
                            UPSMASH_CANCEL_COUNTER[entry_id] = 0;
                            //ENABLE_MULTIHIT_CANCEL[entry_id] = false;
                        }    
                        MOVEMENT_CANCEL[entry_id] = false; 
                    }
                }
            }
            //Resets
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
                UPSMASH_CANCEL_COUNTER[entry_id] = 0;
                //ENABLE_MULTIHIT_CANCEL[entry_id] = false; 
            } */      
        }
            
    }

pub fn install() {
    Agent::new("link")
    .on_line(Main, link_opff)
    .install();

}