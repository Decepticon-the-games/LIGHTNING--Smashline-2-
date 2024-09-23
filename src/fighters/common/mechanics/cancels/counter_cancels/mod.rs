use super::*;

pub unsafe extern "C" fn enable_counter_cancel(fighter : &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

    if entry_id < 8 {

        let rock_paper_scissors = ((cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0)
        || (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0)
        || (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0)
        || (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0));

        if rock_paper_scissors {
            CancelModule::enable_cancel(fighter.module_accessor); 
        }
      
    }                                      
}
