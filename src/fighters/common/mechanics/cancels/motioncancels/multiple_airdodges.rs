use super::*;
use crate::fighters::common::mechanics::lightning_mechanics::lightning_mode::FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING;

pub const AIRDODGE_COUNT: i32 = 0x0102; //  You start off with one airdodge. Every other airdodge after that before touching the ground increases the number up to how many jumps that fighter has.

unsafe extern "C" fn multiple_airdodges(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);   
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(fighter.module_accessor);

        //Reset Airdodge count when u land
        if situation_kind == *SITUATION_KIND_GROUND { 
            WorkModule::set_int(fighter.module_accessor, 0, AIRDODGE_COUNT) 
        }  
        
        //println!("airdodge count: {}", WorkModule::get_int(fighter.module_accessor, AIRDODGE_COUNT)  );
    }
}
#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_flag )]
pub unsafe fn is_flag_replace(module_accessor: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    let ret = original!()(module_accessor, flag);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(module_accessor);
    let max_jumps = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    let edgde_one_wing_max_jumps = WorkModule::get_int(module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_JUMP_COUNT_MAX);    


    if (max_jumps == 2 && WorkModule::get_int(module_accessor, AIRDODGE_COUNT)  <2)
    || (max_jumps == 3 && WorkModule::get_int(module_accessor, AIRDODGE_COUNT)  <3) 
    || (max_jumps == 4 && WorkModule::get_int(module_accessor, AIRDODGE_COUNT)  <4) 
    || (max_jumps == 5 && WorkModule::get_int(module_accessor, AIRDODGE_COUNT)  <5) 
    || (max_jumps == 6 && WorkModule::get_int(module_accessor, AIRDODGE_COUNT)  <6)
    || (edgde_one_wing_max_jumps == 3 && WorkModule::get_int(module_accessor, AIRDODGE_COUNT)  <3)
    {
        if flag == *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR {

            if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                if WorkModule::is_flag(module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING) {//enable 2nd airdodge throughout lightning mode
                    return false; 
                }
                else {
                    return true;
                }
            }
            else {
                return false;
            }
        }
        else {
            return ret;
        }
    } 
    else {
        return ret;
    }     
}
pub fn install() {
    skyline::install_hook!(is_flag_replace);

    Agent::new("fighter")
    .on_line(Main, multiple_airdodges)
    .install()
}