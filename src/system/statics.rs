use super::*;

pub static debug: Lazy<Mutex<debug::debug_master>> = Lazy::new(|| Mutex::new(debug::debug_master::new()));

pub static data: Lazy<Mutex<data::data_master>> = Lazy::new(|| Mutex::new(data::data_master::new(data::player::obj_player::new(1, None))));

pub static cache: Lazy<Mutex<cache::cache_master>> = Lazy::new(|| Mutex::new(cache::cache_master::new()));