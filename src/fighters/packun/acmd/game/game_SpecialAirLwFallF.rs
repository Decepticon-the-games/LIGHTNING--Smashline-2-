use super::*;

unsafe extern "C" fn game_specialairlwfallf(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 7.0, 3.0);
        whiff_cancel(fighter);
    }
}    

pub fn install() {
    Agent::new("packun")
        .game_acmd("game_specialairlwfallf", game_specialairlwfallf, Priority::Low)
        .install();
}
