//CONVERT ALL DEADFALL LINES:
DEADFALL[entry_id] =  *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI;

//find: DEADFALL[entry_id] =
//replace: WorkModule::set_int(fighter.module_accessor, 0, DEADFALL);
//find: *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI; or whatever name it is
//cut
//find: WorkModule::set_int(fighter.module_accessor, 0, DEADFALL);
//replace: WorkModule::set_int(fighter.module_accessor *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI; or whatever name it is, DEADFALL);

WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, DEADFALL);
