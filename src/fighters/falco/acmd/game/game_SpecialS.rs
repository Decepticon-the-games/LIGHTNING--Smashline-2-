use super::*;

unsafe extern "C" fn game_specials(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
let inputs = ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 ) 
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 )
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 )
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 )
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 );

    if macros::is_excute(fighter) {
        if inputs {
            JostleModule::set_status(fighter.module_accessor, true);
            enable_attack_cancel(fighter);
            macros::ATTACK(fighter, 1, 0, Hash40::new("body"), 0.0, 361, 0, 0, 0, 3.0, 0.0, 5.0, 10.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        else {
            JostleModule::set_status(fighter.module_accessor, false);  
        }       
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ILLUSION, false, -1);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ILLUSION, false, -1);
    }
}

pub fn install() {
    Agent::new("falco")
        .game_acmd("game_specials", game_specials, Priority::Low)
        .install();
}
