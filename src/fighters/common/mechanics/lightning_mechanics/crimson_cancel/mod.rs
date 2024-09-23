use super::*;
pub const CRIMSON_CANCEL_TIMER: i32 = 120;
pub const FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CRIMSON_CANCEL: i32 = 0x1E00003F +1;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_CRIMSON_CANCEL: i32 = 0x200000AF + 1;
pub const FIGHTER_INSTANCE_WORK_ID_INT_CRIMSON_CANCEL_TIMER: i32 = 0x100000AF + 1;

static mut CAN_CRIMSON_CANCEL : [bool; 8] = [true; 8];//the ability to crimson cancel
static mut CAN_CRIMSON_CANCEL_TEMP : [bool; 8] = [true; 8];

// CRIMSON CANCELLING: For 2 seconds the opponent will slow down by 5 times, two players can't use them at the same time.
// Can't spark while being hit in hitlag or being held in a grab/throw, wastes it 

unsafe extern "C" fn crimson_cancel_opff(fighter : &mut L2CFighterCommon) {
    let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);  

    if crimson_cancel_conditions(fighter) {
        WorkModule::enable_transition_term(fighter.module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CRIMSON_CANCEL)
    }
    else {
        WorkModule::unable_transition_term(fighter.module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CRIMSON_CANCEL)
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CRIMSON_CANCEL)  {
        if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0 {
            crimson_cancel(fighter); 
        }
    }
    //println!("cc_timer: {}", WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CRIMSON_CANCEL_TIMER));
}
unsafe extern "C" fn crimson_cancel_conditions(fighter : &mut L2CFighterCommon) -> bool {
    ! WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CRIMSON_CANCEL)
    && CancelModule::is_enable_cancel(fighter.module_accessor)
    && DamageModule::damage(fighter.module_accessor, 0) >= 50.0 
    // WHEN THE METER COMES, CHANGE IT TO METER IS +50??   
    && ! (CaptureModule::is_capture(fighter.module_accessor))
    && ! (StopModule::is_hit(fighter.module_accessor))
}
unsafe extern "C" fn crimson_cancel(fighter : &mut L2CFighterCommon) {  
    //println!("cc_flag: {}", WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CRIMSON_CANCEL));
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CRIMSON_CANCEL);
    WorkModule::set_int(fighter.module_accessor, CRIMSON_CANCEL_TIMER, FIGHTER_INSTANCE_WORK_ID_INT_CRIMSON_CANCEL_TIMER);
    crimson_cancel_effects(fighter);
    crimson_cancel_timer(fighter);
}
unsafe extern "C" fn crimson_cancel_timer(fighter : &mut L2CFighterCommon) {
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CRIMSON_CANCEL_TIMER) > 0 {            
        WorkModule::dec_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CRIMSON_CANCEL_TIMER);
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CRIMSON_CANCEL_TIMER) == 0 
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_DEAD 
        || (WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CRIMSON_CANCEL_TIMER) >=1 && AttackModule::is_attack_occur(fighter.module_accessor)) 
        {
            crimson_cancel_disable(fighter);
        }
    } 
}
unsafe extern "C" fn crimson_cancel_effects(fighter : &mut L2CFighterCommon) {
    //Visual Effects    
    EffectModule::req_emit(fighter.module_accessor, smash::phx::Hash40::new("sys_aura_dark"), 0);
    macros::LAST_EFFECT_SET_COLOR(fighter, 0.773, 0.031, 0.304);
    macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 12, 0.1, 0.1, 0.1, 0.01, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
    macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    macros::SLOW_OPPONENT(fighter, 5.0, CRIMSON_CANCEL_TIMER as f32); 
    //Dimensional Effects
    //WorkModule::on_flag(fighter.fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);// gravity
    // CHANGE THE STAGE BACKGROUND!!!!!!!!! 
    // MAKE THE STAGE ITSELF TRANSPARENT!!!!!!!!!!
    //_________________________________________________________________________________________________________________________________________________________________________________    
}
unsafe extern "C" fn crimson_cancel_disable(fighter : &mut L2CFighterCommon) {  
    //When you attack next, the timer runs out or you get KO'd, the effects wear off

    WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CRIMSON_CANCEL);
    macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    macros::CANCEL_FILL_SCREEN(fighter, 0, 5.0);
    macros::SLOW_OPPONENT(fighter, 0.0, 0.0);
    macros::EFFECT_OFF_KIND(fighter, smash::phx::Hash40::new("sys_aura_dark"), true, true);
    WorkModule::set_int(fighter.module_accessor, -1, FIGHTER_INSTANCE_WORK_ID_INT_CRIMSON_CANCEL_TIMER);
}
unsafe extern "C" fn crimson_cancel_resets(fighter : &mut L2CFighterCommon) {
    crimson_cancel_disable(fighter);
}
pub fn install() {
    Agent::new("fighter")
        .on_line(Main, crimson_cancel_opff)
        .on_line(Main, crimson_cancel_timer)
        .on_start(crimson_cancel_resets)
        .install();

} 