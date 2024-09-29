use super::*;

pub trait gmComp{}

pub struct gmObj{
    ID: u16
}

mod tests{
    use super::*;

    pub fn main(){
        let mut hpVec:Vec<gmComp_Health> = Vec::new();
        hpVec.push(gmComp_Health{val: 100});

        let mut objVec: Vec<gmObj> = Vec::new();
        objVec.push(gmObj{ID: 0});
    }

    struct gmComp_Health{
        val: u16
    }
    impl gmComp for gmComp_Health{}
}