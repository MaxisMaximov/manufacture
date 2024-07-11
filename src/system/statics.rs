use super::*;

pub static debug: Lazy<Mutex<debug::debug_master>> = Lazy::new(|| Mutex::new(debug::debug_master::new()));

pub static data: Lazy<Mutex<data::DATA_master>> = Lazy::new(|| Mutex::new(data::DATA_master::new(data::player::TEMPLATE_player::new(1, None))));