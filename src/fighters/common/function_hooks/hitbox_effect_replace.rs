use super::*;
#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
	// Declare boma & fighter_kind
	//let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
	//let fighter_kind = smash::app::utility::get_kind(boma);
	
	// Get hitbox params
    let mut l2c_agent = L2CAgent::new(lua_state);
    let hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
	l2c_agent.clear_lua_stack();
	
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;    
	
	/*for i in 0..36 {
		if i == 32 {
			// in the first new_int function, 1. get the hash of the type you want to check for from
			// https://raw.githubusercontent.com/ultimate-research/param-labels/master/ParamLabels.csv
			// then convert it to decimal. Get the hash and convert it for the one you want it to be replaced with
			// and put it in the second new_int function. If you wanna have, say, all with collision_attr_fire do double damage,
			// you can run the if hitbox_params[32].get_int() == ... line earlier, and in the if statement,
			// set a variable to 2x the value of hitbox_params[3], and then in THIS if statement, replace i == 32 with
			// i == 3, and then have the variable you set to 2x [3] get pushed into a stack with new_num instead of new_int.
			// if you don't know which i corresponds to which part of a hitbox, find a hitbox, and count beginning with "fighter" as -1 (YES, NEGATIVE 1- THE LAST ONE SHOULD BE 35)
			if hitbox_params[32].get_int() == L2CValue::new_int(0x15a2c502b3).get_int() {
				l2c_agent.push_lua_stack(&mut L2CValue::new_int(0x1474a84539));
			} else {
				l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
			}
		}
		else {
			l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
		}
    }*/
	let new_bkb = sv_math::rand(hash40("fighter"), 10) as u64;
	let new_kbg = sv_math::rand(hash40("fighter"), 20) as u64;

	
		for i in 0..36 {
			if i == 7 {//bkb
				l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_bkb));
			}
			else if i == 5{//kbg
				l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_kbg));
			}
			else {
				l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
			}
		}	
	
    return original!()(lua_state);
}
#[skyline::hook(replace = sv_animcmd::ATTACK_ABS)]
unsafe fn grab_replace(lua_state: u64) {
	let mut l2c_agent = L2CAgent::new(lua_state);
    let hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
	l2c_agent.clear_lua_stack();
	
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

	let new_bkb = sv_math::rand(hash40("fighter"), 20) as u64;
	let new_kbg = sv_math::rand(hash40("fighter"), 70) as u64;
		
	for i in 0..15 {
		if THROW_CANCEL[entry_id] 
		{
			if i == 6{//bkb
				l2c_agent.push_lua_stack(&mut L2CValue::new_int(30));
			}
			else if i == 4{//kbg
				l2c_agent.push_lua_stack(&mut L2CValue::new_int(90));
			}
			else {
				l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
			}
		}
		else {
			l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
		}
	}
    return original!()(lua_state);
}
/*
#[skyline::hook(offset = 0x46ae64)]
unsafe fn hit_module_handle_attack_event(pos_x: f32, pos_y: f32, hitbox_id: i32, attacker_id: u32, defender_id: u32)  {
	
}*/
pub fn install() {
    skyline::install_hooks!(
        //attack_replace, 
		grab_replace
    );
}