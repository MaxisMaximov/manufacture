use super::*;

pub trait gmComp{}

mod tests{
    use super::*;

    pub fn main(){
        let mut hpVec:Vec<gmComp_Health> = Vec::new();
        hpVec.push(gmComp_Health{val: 100});
    }

    struct gmComp_Health{
        val: u16
    }
    impl gmComp for gmComp_Health{}
}