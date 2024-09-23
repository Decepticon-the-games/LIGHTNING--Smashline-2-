use super::*;

unsafe extern "C" fn game_speciallwreflect(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD, false, -1);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(fighter, 0.6);
    frame(fighter.lua_state_agent, 35.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}    

pub fn install() {
    Agent::new("palutena")
        .game_acmd("game_speciallwreflect", game_speciallwreflect, Priority::Low)
        .install();
}
