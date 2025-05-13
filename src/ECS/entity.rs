use super::vars::gmID;

pub struct Entity{
    id: gmID,
    hash: u32    
}
impl Entity{
    pub fn new(Id: gmID) -> Self{
        Self{
            id: Id,
            hash: rand::random(),
        }
    }
}

pub struct Token{
    id: gmID,
    hash: u32,
    valid: bool
}
