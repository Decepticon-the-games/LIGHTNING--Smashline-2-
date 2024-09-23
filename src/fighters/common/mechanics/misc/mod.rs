use super::*;

pub mod edgecancelling;
//pub mod moonwalking;


pub fn install() {
    edgecancelling::install();
    //moonwalking::install();
}

