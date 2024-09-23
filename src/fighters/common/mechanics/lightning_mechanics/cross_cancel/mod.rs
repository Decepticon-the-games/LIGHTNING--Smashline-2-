use super::*;

//CROSS CANCEL needs is own button, CONTROL_PAD_BUTTON_CROSS_CANCEL, so people can set it to whaever button they want, 
//combining two existing ones like shield and jump, and it's own cat flag, for the buffer ofc.
//smashline 2 sjould convert all this into it's own , transition term and status kind.

pub static mut CROSS_CANCEL_SETUP : [bool; 8] = [false; 8];
pub static mut CROSS_CANCEL : [bool; 8] = [false; 8];
pub static mut OPPONENT_X : [f32; 8] = [0.0; 8];
pub static mut OPPONENT_Y : [f32; 8] = [0.0; 8];
pub static mut OPPONENT_BOMA  : [u64; 8] = [0; 8];
pub static mut WHO_GOT_HIT_BOMA : [u32; 8] = [0; 8];
pub static mut WHO_HIT_YOU_BOMA : [u32; 8] = [0; 8];
static mut YOU_X : [f32; 8] = [0.0; 8];
static mut YOU_Y : [f32; 8] = [0.0; 8];
static mut SEC_SEN_TIMER : [f32; 8] = [-1.0; 8]; // I start this as -0.4 so that Ryu doesn't immediately start dodging, there's a little pause before he does
static mut OPPONENT_DIRECTION : [f32; 8] = [0.0; 8];
static mut VERT_EXTRA : [f32; 8] = [0.0; 8];
static mut SEC_SEN_DIREC : [i32; 8] = [0; 8];
static mut FLASH_TIMER : [i16; 8] = [-1; 8];

pub const FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CROSS_CANCEL: i32 = 0x1E00003F +2;
pub const FIGHTER_STATUS_KIND_CROSS_CANCEL_START: i32 = 0x8F +2;
pub const FIGHTER_STATUS_KIND_CROSS_CANCEL: i32 = 0x9F +2;
pub const FIGHTER_PAD_CMD_CAT1_FLAG_CROSS_CANCEL: i32 = 0x400000 +2;

//ULTRA INSTINCT

unsafe extern "C" fn cross_cancel_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);

        if entry_id < 1 {
            //println!("va_x: {}", VA_OPPONENT_X[entry_id]);
            //println!("va_y: {}", VA_OPPONENT_Y[entry_id]);
            //println!("cc: {}", SEC_SEN_TIMER[entry_id] );
            //println!("v: {}", ControlModule::get_command_life(fighter.module_accessor, 0, FIGHTER_PAD_CMD_CAT1_FLAG_VANISH));
        }  
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND 
            && CancelModule::is_enable_cancel(fighter.module_accessor)
            {
                WorkModule::enable_transition_term(fighter.module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CROSS_CANCEL)
            }
            else {
                WorkModule::unable_transition_term(fighter.module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CROSS_CANCEL)
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CROSS_CANCEL) {
            {
                if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_STATUS_KIND_CROSS_CANCEL_START, false);
                }
            }        

        }
    }
}
unsafe extern "C" fn cross_cancel_start_pre(fighter : &mut L2CFighterCommon) -> L2CValue {


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
unsafe extern "C" fn cross_cancel_start_end(fighter : &mut L2CFighterCommon) -> L2CValue {
    0.into()
}
unsafe extern "C" fn cross_cancel_start_main(fighter : &mut L2CFighterCommon) -> L2CValue {

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("wait_1"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(cross_cancel_start_main_status_loop as *const () as _))
}
unsafe extern "C" fn cross_cancel_start_main_status_loop(fighter : &mut L2CFighterCommon) -> L2CValue {
    
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;


    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
    }
    if MotionModule::frame(fighter.module_accessor) == 4.0 {
        EffectModule::req_emit(fighter.module_accessor, Hash40::new("sys_aura_light"), 0);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
        macros::PLAY_SE(fighter, Hash40::new("se_ryu_6c_aura"));
        
        FLASH_TIMER[entry_id] = -1;
    }
    if MotionModule::frame(fighter.module_accessor) <= 30.0
    && MotionModule::frame(fighter.module_accessor) >= 4.0 {
        CROSS_CANCEL_SETUP[entry_id] = true;
        //DamageModule::set_damage_lock(fighter.module_accessor, true);
        //DamageModule::set_no_reaction_no_effect(fighter.module_accessor, true);
        //HitModule::set_hit_stop_mul(fighter.module_accessor, 0.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
    }
    else {
        CROSS_CANCEL_SETUP[entry_id] = false;
        //DamageModule::set_damage_lock(fighter.module_accessor, false);
        //DamageModule::set_no_reaction_no_effect(fighter.module_accessor, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), true, true);
        //HitModule::set_hit_stop_mul(fighter.module_accessor, 1.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
        //macros::COL_NORMAL(fighter);
        //macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);  
        //HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }

    // Handles the blue flashes
    if CROSS_CANCEL_SETUP[entry_id] { 
        if FLASH_TIMER[entry_id] < 0 {
            FLASH_TIMER[entry_id] = 8;
        }
        if FLASH_TIMER[entry_id] <= 4 {
            macros::COL_NORMAL(fighter);
            FLASH_TIMER[entry_id] -= 1;
        }
        if FLASH_TIMER[entry_id] > 4 {
            macros::FLASH(fighter, 0, 0.55, 1, 1.75);
            FLASH_TIMER[entry_id] -= 1;
        }
    }
    0.into()
}

unsafe extern "C" fn cross_cancel_pre(fighter : &mut L2CFighterCommon) -> L2CValue {


    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_KEEP as u32,
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
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
unsafe extern "C" fn cross_cancel_init(fighter : &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    cross_cancel_get_current_position(fighter);
    enable_cross_cancel_effects(fighter);
    SEC_SEN_TIMER[entry_id] = -0.2;
    CROSS_CANCEL_SETUP[entry_id] = false;
    0.into()
}
unsafe extern "C" fn cross_cancel_main(fighter : &mut L2CFighterCommon) -> L2CValue {


    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
    //MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape"), 0.0, 1.0, false, 0.0, false, false);
 

    /*if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        if SEC_SEN_DIREC[entry_id] == *FIGHTER_STATUS_KIND_ESCAPE {
            SEC_SEN_DIREC[entry_id] = *FIGHTER_STATUS_KIND_ESCAPE_AIR;
        }
    }*/

    if (YOU_Y[entry_id] - OPPONENT_Y[entry_id]).abs() <= 12.0 
    && StatusModule::situation_kind(OPPONENT_BOMA [entry_id] as *mut smash::app::BattleObjectModuleAccessor) == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    if StatusModule::status_kind(fighter.module_accessor) != SEC_SEN_DIREC[entry_id] { 
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
        //StatusModule::change_status_request_from_script(fighter.module_accessor, SEC_SEN_DIREC[entry_id], true);
    }    
    

    if SEC_SEN_TIMER[entry_id] >= 0.0 { 
        if (YOU_Y[entry_id] - OPPONENT_Y[entry_id]).abs() > 12.0 { 
            StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), true); 
        }
        PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f{ 
            x: (((OPPONENT_X[entry_id] + OPPONENT_DIRECTION[entry_id]) * (SEC_SEN_TIMER[entry_id])) + YOU_X[entry_id] * (1.0 - SEC_SEN_TIMER[entry_id])),
            y: (((OPPONENT_Y[entry_id] + VERT_EXTRA[entry_id]) * SEC_SEN_TIMER[entry_id]) + YOU_Y[entry_id] * (1.0 - SEC_SEN_TIMER[entry_id])) 
        });
        SEC_SEN_TIMER[entry_id] += 0.025;
    }  
    else {
        SEC_SEN_TIMER[entry_id] += 0.1;
    }  
    if SEC_SEN_TIMER[entry_id] > 1.0 {
        disable_cross_cancel_effects(fighter);
        SEC_SEN_TIMER[entry_id] = 1.0; 
    }
    //fighter.sub_shift_status_main(L2CValue::Ptr(cross_cancel_main_loop as *const () as _)) 
    0.into()
}
unsafe extern "C" fn enable_cross_cancel_effects(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
        let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z:90.0}; 

        EffectModule::req_emit(fighter.module_accessor, Hash40::new("sys_aura_light"), 0);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
        DamageModule::set_damage_lock(fighter.module_accessor, true); 
        DamageModule::set_no_reaction_no_effect(fighter.module_accessor, true); 
        HitModule::set_hit_stop_mul(fighter.module_accessor, 0.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0); 
        //macros::PLAY_SE(fighter, Hash40::new("se_ryu_6c_exec"));
        macros::CAM_ZOOM_IN_arg5(fighter, 3.0, 0.0, 1.5, 0.0, 0.0); 
        macros::SLOW_OPPONENT(fighter, 999.0, 1.0); 
        SlowModule::set_whole(fighter.module_accessor, 4, SEC_SEN_TIMER[entry_id] as i32); 
        macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 12, 0.1, 0.1, 0.1, 0, 0.001, 0.011, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);                    
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
}
unsafe extern "C" fn disable_cross_cancel_effects(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
        let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z:90.0};
        
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE); 
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK); 
        macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            PostureModule::reverse_lr(fighter.module_accessor); 
        }
        macros::CAM_ZOOM_OUT(fighter); 
        macros::CANCEL_FILL_SCREEN(fighter, 0, 5); 
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), true, true);
        SlowModule::clear_whole(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        JostleModule::set_status(fighter.module_accessor, true);
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
    }
}
unsafe extern "C" fn cross_cancel_get_current_position(fighter : &mut L2CFighterCommon) {
    
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let opponent_boma = sv_battle_object::module_accessor(WHO_HIT_YOU_BOMA[entry_id]);
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);

    YOU_X[entry_id] = PostureModule::pos_x(fighter.module_accessor);
    YOU_Y[entry_id] = PostureModule::pos_y(fighter.module_accessor);

    if YOU_X[entry_id] == OPPONENT_X[entry_id] { 
        OPPONENT_DIRECTION[entry_id] = -12.0 * PostureModule::lr(fighter.module_accessor);
        /*if fighter_kind == *FIGHTER_KIND_RYU
        || fighter_kind == *FIGHTER_KIND_KEN {
            SEC_SEN_DIREC[entry_id] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B;
        }
        else {
            SEC_SEN_DIREC[entry_id] = *FIGHTER_STATUS_KIND_ESCAPE;
        }*/
    }
    else if YOU_X[entry_id] < OPPONENT_X[entry_id] {
        OPPONENT_DIRECTION[entry_id] = 12.0;
        if PostureModule::lr(fighter.module_accessor) == -1.0 {
            PostureModule::reverse_lr(fighter.module_accessor);
        }
        /*if fighter_kind == *FIGHTER_KIND_RYU
        || fighter_kind == *FIGHTER_KIND_KEN {
            SEC_SEN_DIREC[entry_id] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F;
        }
        else {
            SEC_SEN_DIREC[entry_id] = *FIGHTER_STATUS_KIND_ESCAPE;
        }*/
    }
    else {
        OPPONENT_DIRECTION[entry_id] = -12.0;
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            PostureModule::reverse_lr(fighter.module_accessor);
        }
        /*if fighter_kind == *FIGHTER_KIND_RYU
        || fighter_kind == *FIGHTER_KIND_KEN {
            SEC_SEN_DIREC[entry_id] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F;
        }
        else {
            SEC_SEN_DIREC[entry_id] = *FIGHTER_STATUS_KIND_ESCAPE;
        }*/
    }
}
unsafe extern "C" fn cross_cancel_resets(fighter : &mut L2CFighterCommon) {
}
pub fn install() {
    Agent::new("fighter")
    .on_line(Main, cross_cancel_opff)
    .status(Pre, FIGHTER_STATUS_KIND_CROSS_CANCEL_START, cross_cancel_start_pre)
    .status(Main, FIGHTER_STATUS_KIND_CROSS_CANCEL_START, cross_cancel_start_main)
    .status(End, FIGHTER_STATUS_KIND_CROSS_CANCEL_START, cross_cancel_start_end)

    .status(Pre, FIGHTER_STATUS_KIND_CROSS_CANCEL, cross_cancel_pre)
    .status(Init, FIGHTER_STATUS_KIND_CROSS_CANCEL, cross_cancel_init)
    .status(Main, FIGHTER_STATUS_KIND_CROSS_CANCEL, cross_cancel_main)
    .on_start(cross_cancel_resets)
    .install();
}