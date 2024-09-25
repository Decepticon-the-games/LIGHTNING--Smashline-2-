use super::*;

extern "C"{
    #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__get_fighter_entry_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
    pub fn get_fighter_entry(manager: *mut smash::app::FighterManager, entry_id: u32) -> *mut u8;
}

pub fn get_battle_object_from_entry_id(entry_id: u32) -> Option<*mut BattleObject> {
    unsafe {
        let entry = get_fighter_entry(crate::utils::singletons::FighterManager(), entry_id);
        if entry.is_null() {
            None
        } 
        else {
            Some(*(entry.add(0x4160) as *mut *mut BattleObject))
        }
    }
}