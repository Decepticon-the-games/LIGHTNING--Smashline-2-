use super::*;

pub static UI_MANAGER: Lazy<RwLock<UiManager>> = Lazy::new(|| {
    RwLock::new(
        UiManager {
            lightning_meter: [LightningMeter::default(); 8],
        }
    )}
);

#[repr(C)]
pub struct UiManager {
    pub lightning_meter: [LightningMeter; 8],
}

impl UiManager {
    fn get_ui_index_from_entry_id(entry_id: u32) -> u32 {
        let mut index = 0;
        for n in 0..entry_id {
            if get_battle_object_from_entry_id(n).is_some() {
                index += 1;
            }
        }
        return index;
    }
    //Lightning Meter
    #[export_name = "UiManager__set_lightning_meter_enable"]
    pub extern "C" fn set_lightning_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        manager.lightning_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_enable(enable);
    }
    #[export_name = "UiManager__set_lightning_meter_info"]
    pub extern "C" fn set_lightning_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32) {
        let mut manager = UI_MANAGER.write();
        manager.lightning_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_meter_info(current, max, per_level);
    }
    #[export_name = "UiManager__reset_lightning_meter"]
    pub extern "C" fn reset_lightning_meter(entry_id: u32) {
        let mut manager = UI_MANAGER.write();
        manager.lightning_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].reset();
    }
    #[export_name = "UiManager__change_lightning_meter_color_green"]
    pub extern "C" fn change_lightning_meter_color_green(entry_id: u32) {
        let mut manager = UI_MANAGER.write();
        manager.lightning_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].change_color_green();
    }
    #[export_name = "UiManager__change_lightning_meter_color_purple"]
    pub extern "C" fn change_lightning_meter_color_purple(entry_id: u32) {
        let mut manager = UI_MANAGER.write();
        manager.lightning_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].change_color_purple();
    }
}