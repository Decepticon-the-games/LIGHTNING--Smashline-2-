use {
    smash::{
        app::{
            BattleObject,
            BattleObjectWorld,
            BattleObjectSlow,
            BattleObjectManager,
            BossManager,
            FighterBayonettaFinalModule,
            FighterCutInManager,
            FighterManager,
            FighterParamAccessor2,
            FighterPitBFinalModule,
            GimmickEventPresenter,
            ItemManager,
            ItemParamAccessor,
            StageManager
        },
    },
    skyline::nn::ro::LookupSymbol,
    std::sync::Once,
    once_cell::sync::Lazy,
    parking_lot::RwLock,   
};


pub mod ui;
pub mod utils;
pub mod singletons;

pub fn install() {
    ui::install();
}