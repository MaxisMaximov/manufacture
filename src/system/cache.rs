use super::*;

pub struct cache_master{
    pub inner: HashMap<String, cache_item>
}
impl cache_master {
    pub fn new() -> Self{
        Self{
            inner: HashMap::new()
        }
    }
}

/// # Cache type
/// Allows you to store a selected type of cache
///
/// Can be extended for any other cache type you want
pub enum cache_item {
    usize(usize),
    u8(u8),
    vec2(types::vector2),
    vec3(types::vector3),
    interactCode(logic::GAME_interactions),
}