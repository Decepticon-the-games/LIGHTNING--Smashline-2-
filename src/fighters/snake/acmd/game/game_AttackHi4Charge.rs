use super::*;

unsafe extern "C" fn game_attackhi4charge(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_TRENCHMORTAR, Hash40::new("hold"), false, -1.0);
    }  
}  

pub fn install() {
    Agent::new("snake")
        .game_acmd("game_attackhi4charge", game_attackhi4charge, Priority::Low)
        .install();
}
