use super::*;

unsafe extern "C" fn game_speciallwground(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(fighter.lua_state_agent, 12.0);
    /*battle_object();
    methodlib::L2CValue::L2CValue(void*)();
    methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    methodlib::L2CValue::as_pointer()const(*FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *ARTICLE_OPE_TARGET_LAST);
    is_constraint_article();
    if(false){*/
        if macros::is_excute(fighter) {
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    //}  
}  

pub fn install() {
    Agent::new("snake")
        .game_acmd("game_speciallwground", game_speciallwground, Priority::Low)
        .install();
}
