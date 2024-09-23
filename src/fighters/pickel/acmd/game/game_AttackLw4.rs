use super::*;

unsafe extern "C" fn game_attacklw4(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_PICKEL_STATUS_ATTACK_FLOAT_ATTACK_LW4_MELT_LR);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_MELT, false, -1);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        WorkModule::set_float(fighter.module_accessor, -1.0, *FIGHTER_PICKEL_STATUS_ATTACK_FLOAT_ATTACK_LW4_MELT_LR);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_MELT, false, -1);
        whiff_cancel(fighter);
    }
}

pub fn install() {
    Agent::new("pickel")
        .game_acmd("game_attacklw4", game_attacklw4, Priority::Low)
        .install();
}
