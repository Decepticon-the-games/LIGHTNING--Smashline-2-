use super::*;
#[acmd_script( agent = "miigunner", script = ["game_specialairlw1end", "game_specialairlw1end1"], category = ACMD_GAME, low_priority )]
unsafe extern "C" fn game_specialairlw2(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    whiff_cancel(fighter);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairlw2);
}