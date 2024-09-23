use super::*;

pub unsafe extern "C" fn moveset_input(fighter : &mut L2CAgentBase) -> bool {
    unsafe {
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

        (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0
        ||(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0
        ||(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0
        ||(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0
        ||(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0
        ||(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0
        ||(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0
        ||(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0
    }
}
pub unsafe extern "C" fn whiff_input(fighter : &mut L2CAgentBase) -> bool {
    unsafe {
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);

        (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0
        ||(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0
        ||(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 
        ||(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0
        ||(cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0
        ||(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0
        ||(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F) != 0
        ||(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B) != 0
        //||(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_AIR) != 0
    }
}
pub unsafe extern "C" fn cancels(fighter : &mut L2CAgentBase) {
}

pub mod attack_cancels;
pub mod counter_cancels;
pub mod motioncancels;

pub fn install() {

    attack_cancels::install();
    motioncancels::install();
}