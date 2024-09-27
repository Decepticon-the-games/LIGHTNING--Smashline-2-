use super::*;
use crate::utils::ui::UiManager;

unsafe extern "C" fn lightning_meter(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
        //let hit_damage = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE) as f32;

        UiManager::set_lightning_meter_enable(entry_id, true);
        UiManager::set_lightning_meter_info(entry_id, 100.0, 100.0, 50.0);
    }
}
pub fn install() {
    Agent::new("fighter")
    .on_line(Main, lightning_meter)
    .install();
}