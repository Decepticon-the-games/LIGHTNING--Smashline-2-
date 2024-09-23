use super::*;

#[fighter_frame_callback]
pub fn throw_cancel(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;   
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

        //Handling throw cancels here, so that it's only possible to do throw cancels during hitlag.
        if CatchModule::is_catch(fighter.module_accessor)
        && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0
        {
            THROW_CANCEL[entry_id] = true;
            if is_after_hitlag(fighter) {
                THROW_CANCEL[entry_id] = false;
            }
        } 
    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(throw_cancel);
}