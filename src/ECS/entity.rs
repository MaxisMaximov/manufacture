pub struct Entity{
    id: usize,
    hash: u32    
}
impl Entity{
    pub fn new(Id: usize) -> Self{
        Self{
            id: Id,
            hash: rand::random(),
        }
    }
}

pub struct Token{
    id: usize,
    hash: u32,
    valid: bool
}
