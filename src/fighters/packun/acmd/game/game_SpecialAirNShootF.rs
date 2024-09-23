use super::*;

unsafe extern "C" fn game_specialairnshootf(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 7.0, 0.0, 0.0, 8.0, 0.0, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(fighter) {
            macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 3.0, 0.0, 0.0, 3.0, 0.0, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
        }
    }
    frame(fighter.lua_state_agent, 1.0);
    WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_SHOOT_PREVIOUS_WAIT);
    if(methodlib::L2CValue::operator==(lib::L2CValueconst&)const(false, true)){
        WorkModule::get_float(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_WORK_FLOAT_SHOOT_START_SPIKEBALL_DISTANCE);
        PostureModule::scale(fighter.module_accessor, 0);
        0x1646b0(0, 30);
        if(0x1646b0()){
            macros::FT_MOTION_RATE(fighter, 0.6);
            frame(fighter.lua_state_agent, 8.0);
            if macros::is_excute(fighter) {
                ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_PACKUN_GENERATE_ARTICLE_SPIKEBALL, *ARTICLE_OPE_TARGET_ALL, false);
            }
            frame(fighter.lua_state_agent, 10.0);
            if macros::is_excute(fighter) {
                search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
            }
            frame(fighter.lua_state_agent, 11.0);
            macros::FT_MOTION_RATE(fighter, 2.0);
            frame(fighter.lua_state_agent, 12.0);
            macros::FT_MOTION_RATE(fighter, 0.5);
            if macros::is_excute(fighter) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_SHOOT_BLOW_SPIKEBALL);
            }
            frame(fighter.lua_state_agent, 19.0);
            macros::FT_MOTION_RATE(fighter, 1.4);
            if macros::is_excute(fighter) {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_SHOOT_BLOW_SPIKEBALL);
            }
            else{
            macros::FT_MOTION_RATE(fighter, 1.0);
            frame(fighter.lua_state_agent, 10.0);
            if macros::is_excute(fighter) {
                search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
            }
            frame(fighter.lua_state_agent, 11.0);
            macros::FT_MOTION_RATE(fighter, 3.0);
            if macros::is_excute(fighter) {
                ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_PACKUN_GENERATE_ARTICLE_SPIKEBALL, *ARTICLE_OPE_TARGET_ALL, false);
            }
            frame(fighter.lua_state_agent, 12.0);
            macros::FT_MOTION_RATE(fighter, 1.0);
            if macros::is_excute(fighter) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_SHOOT_BLOW_SPIKEBALL);
            }
            frame(fighter.lua_state_agent, 19.0);
            macros::FT_MOTION_RATE(fighter, 1.1);
            if macros::is_excute(fighter) {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_SHOOT_BLOW_SPIKEBALL);
            }
        }
        frame(fighter.lua_state_agent, 31.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}    
WorkModule::get_float(fighter.module_accessor, 0, *FIGHTER_PACKUN_STATUS_SPECIAL_N_WORK_FLOAT_SHOOT_START_SPIKEBALL_DISTANCE);
PostureModule::scale(fighter.module_accessor, 0);
0x1646b0(0, 40);
if(0x1646b0()){
    macros::FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_PACKUN_GENERATE_ARTICLE_SPIKEBALL, *ARTICLE_OPE_TARGET_ALL, false);
    }
}    
frame(fighter.lua_state_agent, 10.0);
if macros::is_excute(fighter) {
    search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
}
frame(fighter.lua_state_agent, 11.0);
macros::FT_MOTION_RATE(fighter, 3.0);
frame(fighter.lua_state_agent, 12.0);
macros::FT_MOTION_RATE(fighter, 0.7);
if macros::is_excute(fighter) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_SHOOT_BLOW_SPIKEBALL);
}
frame(fighter.lua_state_agent, 19.0);
macros::FT_MOTION_RATE(fighter, 1.3);
if macros::is_excute(fighter) {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_SHOOT_BLOW_SPIKEBALL);
}
else{
WorkModule::get_float(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_WORK_FLOAT_SHOOT_START_SPIKEBALL_DISTANCE);
PostureModule::scale(fighter.module_accessor, 0);
0x1646b0(0, 50);
if(0x1646b0()){
    macros::FT_MOTION_RATE(fighter, 0.9);
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
    }
    frame(fighter.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(fighter, 4.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_PACKUN_GENERATE_ARTICLE_SPIKEBALL, *ARTICLE_OPE_TARGET_ALL, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_SHOOT_BLOW_SPIKEBALL);
    }
    frame(fighter.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(fighter, 1.2);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_SHOOT_BLOW_SPIKEBALL);
    }
    else{
    WorkModule::get_float(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_WORK_FLOAT_SHOOT_START_SPIKEBALL_DISTANCE);
    PostureModule::scale(fighter.module_accessor, 0);
    0x1646b0(0, 100);
    if(0x1646b0()){
        macros::FT_MOTION_RATE(fighter, 1.2);
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
        }
        frame(fighter.lua_state_agent, 11.0);
        macros::FT_MOTION_RATE(fighter, 5.0);
        if macros::is_excute(fighter) {
            ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_PACKUN_GENERATE_ARTICLE_SPIKEBALL, *ARTICLE_OPE_TARGET_ALL, false);
        }
        frame(fighter.lua_state_agent, 12.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_SHOOT_BLOW_SPIKEBALL);
        }
        frame(fighter.lua_state_agent, 19.0);
        macros::FT_MOTION_RATE(fighter, 1.1);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_SHOOT_BLOW_SPIKEBALL);
        }
        else{
        macros::FT_MOTION_RATE(fighter, 1.0);
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
        }
    }
}    
}
}
}
frame(fighter.lua_state_agent, 11.0);
macros::FT_MOTION_RATE(fighter, 5.0);
if macros::is_excute(fighter) {
ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_PACKUN_GENERATE_ARTICLE_SPIKEBALL, *ARTICLE_OPE_TARGET_ALL, false);
}
frame(fighter.lua_state_agent, 12.0);
macros::FT_MOTION_RATE(fighter, 1.0);
if macros::is_excute(fighter) {
WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_SHOOT_BLOW_SPIKEBALL);
}
frame(fighter.lua_state_agent, 19.0);
macros::FT_MOTION_RATE(fighter, 1.0);
WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_SHOOT_BLOW_SPIKEBALL);
}   

pub fn install() {
    Agent::new("packun")
        .game_acmd("game_specialairnshootf", game_specialairnshootf, Priority::Low)
        .install();
}
