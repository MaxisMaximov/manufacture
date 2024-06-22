use super::*;

pub static SYS_errorQueue: Lazy<Mutex<error::MASTER_errorQueue>> = Lazy::new(|| Mutex::new(error::MASTER_errorQueue::new()));

pub static SYS_debug: Lazy<Mutex<debug::DEBUG_master>> = Lazy::new(|| Mutex::new(debug::DEBUG_master::new()));

pub static SYS_data: Lazy<Mutex<data::DATA_master>> = Lazy::new(|| Mutex::new(data::DATA_master::new(data::player::TEMPLATE_player::new(1, None))));