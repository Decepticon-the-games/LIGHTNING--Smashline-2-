use super::*;

//#[fighter_frame( agent = FIGHTER_KIND_DONKEY )]
unsafe extern "C" fn donkey_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
        let frame = MotionModule::frame(fighter.module_accessor);
        
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        

        //Cancel up special only after 3 hits


        //In Lightning...
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING) {
            //Up B cancels after 3 successful hits, cancel into jabs, tilts, smashes, neutral/side b     
            let next_input = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 
            || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0
            || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 
            || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0;
            multihit_counter(fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI, 0, 0, 3, next_input, *FIGHTER_STATUS_KIND_SPECIAL_HI, 0, 0);
        }
    }
}

pub fn install() {
    Agent::new("donkey")
    .on_line(Main, donkey_opff)
    .install();
}