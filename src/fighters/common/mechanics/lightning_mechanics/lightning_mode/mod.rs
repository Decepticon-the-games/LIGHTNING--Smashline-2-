use crate::utils::ui::UiManager;
use super::*;
use skyline::nn::ro::LookupSymbol;
use smash::app::{self, lua_bind::{FighterManager, FighterInformation, *}, sv_animcmd::*, *};

pub const FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_LIGHTNING: i32 = 0x1E00003F +3;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING: i32 = 0x010B;
pub const FIGHTER_INSTANCE_WORK_ID_INT_LIGHTNING_TIMER: i32 = 0x010C;
pub const FIGHTER_STATUS_KIND_LIGHTNING: i32 = 0x8F + 3;

pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
pub static mut CAN_LIGHTNING: [bool; 8] = [true; 8];
pub static mut LIGHTNING_PRE : [bool; 8] = [false; 8];
//pub static mut LIGHTNING_TIMER : [i32; 8] = [-1;  8];
//pub static mut LIGHTNING_TIMER_FS_METER : [i32; 8] = [-1;  8];
pub static mut CAN_LIGHTNING_TEMP : [bool; 8] = [true; 8];
pub static mut LIGHTNING_BUTTON : [bool; 8] = [false; 8];
pub static mut LIGHTNING_EFFECTS: [bool; 8] = [false; 8];
pub static mut ONEFRAMEEFFECTS: [bool; 8] = [false; 8];

unsafe extern "C" fn lightning_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let entry_id_u32 = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        let hitlag = (SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) > 0 || StopModule::is_stop(fighter.module_accessor));
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let idles =  (status_kind == *FIGHTER_STATUS_KIND_WAIT || status_kind == *FIGHTER_STATUS_KIND_JUMP || status_kind == *FIGHTER_STATUS_KIND_FALL);

        if entry_id < 1 {
            //println!("lightning: {}", WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING));
            //println!("lt: {}", WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_LIGHTNING_TIMER) );
        }
        
        UiManager::set_palutena_meter_enable(entry_id_u32, true);
        if lightning_mode_conditions(fighter) {
            WorkModule::enable_transition_term(fighter.module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_LIGHTNING)
        }
        else {
            WorkModule::unable_transition_term(fighter.module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_LIGHTNING)
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_LIGHTNING)  {
            if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_STATUS_KIND_LIGHTNING, false);
            }     
        }
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING) {
            macros::BURN_COLOR(fighter, 0.0, 0.784, 1.0, 0.7); 
            //Countdown
            if WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_LIGHTNING_TIMER) >= 1 {
                if ! (status_kind == *FIGHTER_STATUS_KIND_REBIRTH || status_kind == *FIGHTER_STATUS_KIND_DEAD) {//Will halt countdown on death/respawn
                    WorkModule::dec_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_LIGHTNING_TIMER);
                }
            }
            //STOP LIGHTNING
            if WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_LIGHTNING_TIMER) == 0 {
                lightning_disable(fighter);
            }   
        }
    }
}
unsafe extern "C" fn lightning_mode_conditions(fighter : &mut L2CFighterCommon) -> bool {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        ! WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING) 
        && CancelModule::is_enable_cancel(fighter.module_accessor)
    }
}
unsafe extern "C" fn lightning_timer_countdown(fighter : &mut L2CFighterCommon) {

    LookupSymbol(&mut FIGHTER_MANAGER_ADDR, "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}".as_bytes().as_ptr());
    let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let entry_idi32 = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let dead_count = FighterInformation::dead_count(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(entry_idi32)),0);
    
    //timers
    if dead_count == 0 {
        WorkModule::set_int(fighter.module_accessor, 1200, FIGHTER_INSTANCE_WORK_ID_INT_LIGHTNING_TIMER);
    }
    if dead_count == 1 {
        WorkModule::set_int(fighter.module_accessor, 1800, FIGHTER_INSTANCE_WORK_ID_INT_LIGHTNING_TIMER);
    }
    if dead_count == 2 {
        WorkModule::set_int(fighter.module_accessor, 2400, FIGHTER_INSTANCE_WORK_ID_INT_LIGHTNING_TIMER);
    }
    if dead_count >= 3 {
        WorkModule::set_int(fighter.module_accessor, 3600, FIGHTER_INSTANCE_WORK_ID_INT_LIGHTNING_TIMER);
    }   
      
}
unsafe extern "C" fn status_lightning_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_KEEP as u32,
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn status_lightning_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    lightning_effects(fighter);
    0.into()
}
unsafe extern "C" fn status_lightning_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    //MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING);
    
    lightning_timer_countdown(fighter);

    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }    
    }
    0.into()
}
unsafe extern "C" fn status_lightning_exec(fighter: &mut L2CFighterCommon) -> L2CValue {

    0.into()
}
unsafe extern "C" fn status_lightning_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());

    }
    else if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}


//VISUAL EFFECTS
unsafe extern "C" fn lightning_effects(fighter : &mut L2CFighterCommon) {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    let vector = Vector3f{x:0.0,y:10.0,z:0.0};
    let effect = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_aura_light"), (Hash40::new("top")), &vector, &vector, 3.0, true, 0, 0, 0, 0, 0, true, true) as u32;

    //macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.784, 1.0,);
    EffectModule::set_rgb(fighter.module_accessor, EffectModule::req_emit(fighter.module_accessor, Hash40::new("sys_aura_light"), 0) as u32, 0.0, 0.784, 1.0);
    EffectModule::set_rgb(fighter.module_accessor, EffectModule::req_emit(fighter.module_accessor, Hash40::new("sys_aura_light"), 0) as u32, 0.0, 0.784, 1.0);
    ModelModule::enable_gold_eye(fighter.module_accessor);	 
    //ModelModule::set_color_rgb(fighter.module_accessor, 0.0, 0.784, 1.0, MODEL_COLOR_TYPE::MODEL_COLOR_TYPE_NORMAL);
}
//DISABLE
unsafe extern "C" fn lightning_disable(fighter : &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    
   
    
    macros::BURN_COLOR_NORMAL(fighter);
    macros::COL_NORMAL(fighter);
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_final_aura"), true, true);
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), true, true); 
    ModelModule::disable_gold_eye(fighter.module_accessor);	   
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING);
    
}
//RESETS
unsafe extern "C" fn lightning_resets(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        lightning_disable(fighter);
        CAN_LIGHTNING[entry_id] = true;
    }
}
pub fn install() {
    Agent::new("fighter")
        .on_line(Main, lightning_opff)
        .status(Pre, FIGHTER_STATUS_KIND_LIGHTNING, status_lightning_pre)
        .status(Init, FIGHTER_STATUS_KIND_LIGHTNING, status_lightning_init)
        .status(Main, FIGHTER_STATUS_KIND_LIGHTNING, status_lightning_main)
        .on_start(lightning_resets)
        .install();
} 