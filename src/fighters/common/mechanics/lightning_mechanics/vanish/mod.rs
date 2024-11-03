use super::*;

pub static mut CAN_VANISH : [bool; 8] = [false; 8];//Incorporating the ability to use vanish under condition. See lightning_01_motioncancels/mod.rs
//pub static mut VANISH_COUNT : [i32; 8] = [0; 8];//See motioncancels/mod.rs
pub const VANISH_COUNT: i32 = 0x010B;

//everything to do with opponent information
pub static mut VA_OPPONENT_X : [f32; 8] = [0.0; 8];
pub static mut VA_OPPONENT_Y : [f32; 8] = [0.0; 8];
pub static mut VA_OPPONENT_BOMA  : [u64; 8] = [0; 8];
static mut VA_OPPONENT_DIRECTION_Y : [f32; 8] = [12.0; 8];
static mut VA_OPPONENT_DIRECTION_X : [f32; 8] = [12.0; 8];
pub static mut VA_WHO_GOT_HIT_BOMA : [u32; 8] = [0; 8];
pub static mut VA_WHO_HIT_YOU_BOMA : [u32; 8] = [0; 8];
pub static mut KB_SPEED : [f32; 8] = [0.0; 8];

static mut YOU_X : [f32; 8] = [0.0; 8];
static mut YOU_Y : [f32; 8] = [0.0; 8];
pub static mut VANISH_TIMER : [f32; 8] = [0.0; 8];
pub static mut DEFENDER_VANISH : [bool; 8] = [false; 8];
//pub static mut VANISH_HEIGHT : [f32; 8] = [WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0); 8]; //Gets every character's specific height for vanish position
pub static mut VERT_EXTRA : [f32; 8] = [12.0; 8];

pub static mut WEAPON_BOMA : [u32; 8] = [0; 8];
pub static mut FLOAT : [bool; 8] = [false; 8];
pub static mut FLOAT_TIMER : [i32; 8] = [0; 8];
pub static mut WINDOW : [i32; 8] = [0; 8];

pub const FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_VANISH: i32 = 0x1E00003F +1;
pub const FIGHTER_STATUS_KIND_VANISH: i32 = 0x8F +1;
pub const FIGHTER_PAD_CMD_CAT1_FLAG_VANISH: i32 = 0x400000 +1;
//pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_VANISH_INTERPOLATION_TIMER: i32 = 0x11F + 1;

//CONDITION
unsafe extern "C" fn vanish_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let opponent_boma = sv_battle_object::module_accessor(VA_WHO_GOT_HIT_BOMA[entry_id]);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let opponent_pos_x = PostureModule::pos_x(opponent_boma);
        let opponent_pos_y = PostureModule::pos_y(opponent_boma);
        let pos_x = PostureModule::pos_x(fighter.module_accessor);
        let pos_y = PostureModule::pos_y(fighter.module_accessor);
        //let pos_from_you = ((pos_x * opponent_pos_x) + (pos_y * opponent_pos_y)).sqrt();

        if entry_id < 1 {
            //println!("va_x: {}", VA_OPPONENT_X[entry_id]);
            //println!("va_y: {}", VA_OPPONENT_Y[entry_id]);
            //
            println!("va_count: {}", WorkModule::get_int(fighter.module_accessor, VANISH_COUNT));
        }  
              
        if vanish_condition(fighter) {
            WorkModule::enable_transition_term(fighter.module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_VANISH)
        }
        else {
            WorkModule::unable_transition_term(fighter.module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_VANISH)
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_VANISH)  {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_STATUS_KIND_VANISH, false);
            }
        }
        //Reset count on ground
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            WorkModule::set_int(fighter.module_accessor, 0, VANISH_COUNT);
        }
    }
}
unsafe extern "C" fn vanish_count_max(fighter : &mut L2CFighterCommon) -> bool { 
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    //let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
    let max_jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    let edge_one_wing_max_jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_JUMP_COUNT_MAX);

    WorkModule::get_int(fighter.module_accessor, VANISH_COUNT) < max_jumps
    || WorkModule::get_int(fighter.module_accessor, VANISH_COUNT) < edge_one_wing_max_jumps
}   

unsafe extern "C" fn vanish_condition(fighter : &mut L2CFighterCommon) -> bool {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let opponent_boma = sv_battle_object::module_accessor(VA_WHO_GOT_HIT_BOMA[entry_id]);
    let speed_x = KineticModule::get_sum_speed_x(opponent_boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE);
    let speed_y = KineticModule::get_sum_speed_y(opponent_boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE);
    let opp_kb_speed = ((speed_x*speed_x + speed_y*speed_y)).sqrt();
    let opponent_pos_x = PostureModule::pos_x(opponent_boma);
    let opponent_pos_y = PostureModule::pos_y(opponent_boma);
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    //let pos_from_you = ((pos_x - opponent_pos_x).exp2() + (pos_y - opponent_pos_y).exp2()).sqrt();

    use crate::fighters::common::mechanics::cancels::attack_cancels::cancel_on_hit::MULTIHIT_CANCEL;
    AttackModule::is_attack_occur(fighter.module_accessor)
    && ! WorkModule::is_flag(fighter.module_accessor, MULTIHIT_CANCEL)
    && ! AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
    && CancelModule::is_enable_cancel(fighter.module_accessor)
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR
    && ! WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DEATH_PREDICTION)
    && vanish_count_max(fighter)
    && opp_kb_speed < 9.0
    //&& pos_from_you < 100.0
}
//STATUS
unsafe extern "C" fn status_vanish_pre(fighter: &mut L2CFighterCommon) -> L2CValue {


        StatusModule::init_settings(
            fighter.module_accessor,
            smash::app::SituationKind(*SITUATION_KIND_AIR),
            *FIGHTER_KINETIC_TYPE_NONE,
            *GROUND_CORRECT_KIND_AIR as u32,
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
            *FIGHTER_TREADED_KIND_DISABLE,
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
unsafe extern "C" fn status_vanish_init(fighter: &mut L2CFighterCommon) -> L2CValue {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let dir_x = ControlModule::get_stick_x(fighter.module_accessor);
    let dir_y = ControlModule::get_stick_y(fighter.module_accessor);
    //let stick_degrees = dir_y.atan2(dir_x).to_degrees();         
    let stick_manipulation = dir_x >= 0.8 || dir_x <= -0.8 || dir_y >= 0.8 || dir_y <= -0.8;
    let opponent_boma = sv_battle_object::module_accessor(VA_WHO_GOT_HIT_BOMA[entry_id]);

    WorkModule::inc_int(fighter.module_accessor, VANISH_COUNT);
    VANISH_TIMER[entry_id] = -1.0;
    enable_vanish_effects(fighter);

    YOU_X[entry_id] = PostureModule::pos_x(fighter.module_accessor); //Gets your position
    YOU_Y[entry_id] = PostureModule::pos_y(fighter.module_accessor);
    VA_OPPONENT_BOMA[entry_id] = (&mut *opponent_boma as *mut BattleObjectModuleAccessor) as u64;
    

    //CHECK IF STAGE IS IN BATTLEFIELD/OMEGA FORM, ELSE FIND BOUNDS OF EACH STAGE. PUT THAT IN A FUNCTION.
    //OR CHECK FOR THE BOUNDS OF A BATTLEFIELD/OMEGA FORM STAGE.

    /*if VA_OPPONENT_X[entry_id] <= 120.0
    && VA_OPPONENT_X[entry_id] >= -150.0
    && VA_OPPONENT_Y[entry_id] <= 150.0 
    && VA_OPPONENT_Y[entry_id] >= 70.0 {// IN THE BOUNDS OF A BATTLEFIELD/OMEGA FORM STAGE. 

    }*/
    if stick_manipulation && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {//Manipulate position using a formula converting stickc_degrees to the point of circumference (12x/12y inversion unit radius)

        VA_OPPONENT_DIRECTION_X[entry_id] = dir_x * -12.0; //(stick driection * how far from the opponent's destination)
        VA_OPPONENT_DIRECTION_Y[entry_id] = dir_y * -12.0;                  
        
    }
    else {//If no stick direction, default to facing behind the opponent
        if YOU_X[entry_id] == VA_OPPONENT_X[entry_id] {
            VA_OPPONENT_DIRECTION_X[entry_id] = -12.0 * PostureModule::lr(fighter.module_accessor);                                 
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            VA_OPPONENT_DIRECTION_Y[entry_id] = 0.0;
        }
        else if YOU_X[entry_id] < VA_OPPONENT_X[entry_id] {
            VA_OPPONENT_DIRECTION_X[entry_id] = 12.0;                                   
            if PostureModule::lr(fighter.module_accessor) == 1.0 {
                PostureModule::reverse_lr(fighter.module_accessor);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
            }
            VA_OPPONENT_DIRECTION_Y[entry_id] = 0.0;
        }
        else if YOU_X[entry_id] > VA_OPPONENT_X[entry_id]{
            VA_OPPONENT_DIRECTION_X[entry_id] = -12.0; 
            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                PostureModule::reverse_lr(fighter.module_accessor);
                PostureModule::update_rot_y_lr(fighter.module_accessor);                           
            }
            VA_OPPONENT_DIRECTION_Y[entry_id] = 0.0;
        }                
    }

    ControlModule::set_command_life_extend(fighter.module_accessor, 20 as u8);
    0.into()
}
unsafe extern "C" fn status_vanish_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let opponent_boma = sv_battle_object::module_accessor(VA_WHO_GOT_HIT_BOMA[entry_id]);

    //Just so we don't have to deal with lingering hitboxes from the last animation hehe
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);

    //Incase you're too close to the ground that it resets the count
    StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), true);

    //Get oponent position, update every frame
    VA_OPPONENT_X[entry_id] = PostureModule::pos_x(opponent_boma);
    VA_OPPONENT_Y[entry_id] = PostureModule::pos_y(opponent_boma); 

    VANISH_TIMER[entry_id] += 0.1;

    if VANISH_TIMER[entry_id] >= 0.0 { // This whole if statement is for linearly interpolating your position, instead of just teleporting behind the opponent.
        if (YOU_Y[entry_id] - VA_OPPONENT_Y [entry_id]).abs() > 12.0 { // If yuor Y and the Opponent's Y are far enough apart, do the following:
            StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), false); // Set yourself to be airborne
        }
        PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f{ // Linear Interpolation formula: Destination * t + Starting * (1.0 - t), where 0 <= t <= 1. You can't add vectors apparently, so I did this for both X and Y.
            x: ((VA_OPPONENT_X[entry_id] + VA_OPPONENT_DIRECTION_X[entry_id]) * VANISH_TIMER[entry_id]) + YOU_X[entry_id] * (1.0 - VANISH_TIMER[entry_id]),
            y: ((VA_OPPONENT_Y [entry_id] + VA_OPPONENT_DIRECTION_Y[entry_id] + VERT_EXTRA[entry_id]) * VANISH_TIMER[entry_id]) + YOU_Y[entry_id] * (1.0 - VANISH_TIMER[entry_id])
        });
    }
    if VANISH_TIMER[entry_id] > 1.0 {
        //VANISH_HEIGHT[entry_id] = 0.0;
        VANISH_TIMER[entry_id] = 1.0; //Resets the interpolation timer.                  
        disable_vanish_effects(fighter);
    } 
    0.into()
}
unsafe extern "C" fn status_vanish_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    VANISH_TIMER[entry_id] = 0.0;
    
    0.into()
}
//EFFECTS ON (START)
unsafe extern "C" fn enable_vanish_effects(fighter : &mut L2CFighterCommon) {
    unsafe {
        let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
        let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z:90.0}; 

        KineticModule::clear_speed_all(fighter.module_accessor);//Float
        DamageModule::set_damage_lock(fighter.module_accessor, true);
        DamageModule::set_no_reaction_no_effect(fighter.module_accessor, true); 
        HitModule::set_hit_stop_mul(fighter.module_accessor, 0.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0); 
        JostleModule::set_status(fighter.module_accessor, false); 
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);  
        EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("waist"), &zero, &rotation, 1.2, &zero, &zero, false, 0, 0, 0);
        macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.0, 0.0,);
        macros::BURN_COLOR(fighter, 0.0, 0.0, 0.0, 1.0);
        VisibilityModule::set_whole(fighter.module_accessor, false);     
    }
}
//EFFECTS OFF (END)
unsafe extern "C" fn disable_vanish_effects(fighter : &mut L2CFighterCommon) {
    unsafe {
        let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
        let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z:90.0};
        
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            //FLOAT[entry_id] = true;
        }
        else if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        DamageModule::set_damage_lock(fighter.module_accessor, false);
        DamageModule::set_no_reaction_no_effect(fighter.module_accessor, false);
        HitModule::set_hit_stop_mul(fighter.module_accessor, 1.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
        VisibilityModule::set_whole(fighter.module_accessor, true); 
        EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("waist"), &zero, &rotation, 1.0, &zero, &zero, false, 0, 0, 0);
        macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.0, 0.0,);
        macros::BURN_COLOR(fighter, 0.0, 0.0, 0.0, 1.0);
        JostleModule::set_status(fighter.module_accessor, true);   
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);    
    }
}
unsafe extern "C" fn vanish_resets(fighter : &mut L2CFighterCommon) {
}
pub fn install() {
    Agent::new("fighter")
        .on_line(Main, vanish_opff)
        .status(Pre, FIGHTER_STATUS_KIND_VANISH, status_vanish_pre)
        .status(Init, FIGHTER_STATUS_KIND_VANISH, status_vanish_init)
        .status(Main, FIGHTER_STATUS_KIND_VANISH, status_vanish_main)
        .status(End, FIGHTER_STATUS_KIND_VANISH, status_vanish_end)
        .on_start(vanish_resets)
        .install();
}  
