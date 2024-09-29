use super::*;

pub trait gmComp{}

pub struct gmObj{
    ID: u16
}

pub struct gmWorld{
    gmObjs: Vec<gmObj>,
    hpVec: Vec<tests::gmComp_Health>,
    posVec: Vec<tests::gmComp_Pos>
}

mod tests{
    use super::*;

    pub fn main(){
        let mut world = gmWorld{
            gmObjs: Vec::new(),
            hpVec: Vec::new(),
            posVec: Vec::new(),
        };

        world.hpVec.push(gmComp_Health{val: 100});
        world.posVec.push(gmComp_Pos{x: 0, y: 0});
    }

    pub struct gmComp_Health{
        val: u16
    }
    impl gmComp for gmComp_Health{}

    pub struct gmComp_Pos{
        x: usize,
        y: usize
    }
    impl gmComp for gmComp_Pos{}
}