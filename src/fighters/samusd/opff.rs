use super::*;






//#[fighter_frame( agent = FIGHTER_KIND_SAMUSD )]
pub fn samus_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);      
        
        

//
            static mut CANCEL_COUNT : [bool; 8] = [false; 8];
            static mut CANCEL_COUNTER : [i32; 8] = [0; 8];

            /*if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
            || motion_kind == hash40("attack_air_f")
            || motion_kind == hash40("attack_air_hi")
            {
                //if AttackModule::is_attack_occur(fighter.module_accessor) {
                    //Counter 
                    //if CANCEL_COUNT[entry_id] == false {
                    //    CANCEL_COUNTER[entry_id] +=1;
                    //    CANCEL_COUNT[entry_id] = true; 
                    //}
                //}
                //else {
                //    CANCEL_COUNT[entry_id] = false;
                //}
                //Disable cancel
                //if CANCEL_COUNTER[entry_id] >5 {//How many times you can cancel
                //    CANCEL_COUNTER[entry_id] = 6;//How  many hits before disabling cancel
                //    //ENABLE_MULTIHIT_CANCEL[entry_id] = false; 
                //}

                //Reset
                //if MOVEMENT_CANCEL[entry_id] {
                //    if CANCEL_COUNTER[entry_id] == 6 {
                //        CANCEL_COUNTER[entry_id] = 0;
                //        enable_multihit_cancel(fighter);
                //    }  
                //    MOVEMENT_CANCEL[entry_id] = false;  
                //}
            }

            //Resets
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
                CANCEL_COUNTER[entry_id] = 0;
            }*/
    }
}

pub fn install() {
    Agent::new("mario")
    .on_line(Main, mario_frame)
    .install();(samus_opff);

}