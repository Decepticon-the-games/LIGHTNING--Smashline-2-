use super::*;

pub mod airdash;
//pub mod airstep;
pub mod cancel_in_neutral;
pub mod multiple_airdodges;
//pub mod resets_falses;
//pub mod transitions;
pub mod wavedash;
//pub mod wavestep;

pub fn install() {
    cancel_in_neutral::install();
    airdash::install();
    multiple_airdodges::install();
    wavedash::install();
}