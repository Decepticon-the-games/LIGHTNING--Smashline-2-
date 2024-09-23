use super::*;

unsafe extern "C" fn game_attackairf(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.5);
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: -1.0, y: 0.0, z: 0.0});
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_ATTACKAIRF_BULLET, false, -1);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}    

pub fn install() {
    Agent::new("miigunner")
        .game_acmd("game_attackairf", game_attackairf, Priority::Low)
        .install();
}
