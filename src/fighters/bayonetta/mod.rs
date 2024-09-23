use super::*;

pub mod acmd;
//pub mod status;
pub mod opff;
//pub mod wickedweavearm_opff;
//pub mod wickedweaveleg_opff;
//pub mod params;

pub fn install() {
    acmd::install();
    //status::install();
    opff::install();
    //wickedweavearm_opff::install();
    //wickedweaveleg_opff::install();
    //params::install();
}