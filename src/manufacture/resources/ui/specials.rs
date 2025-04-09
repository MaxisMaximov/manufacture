use super::*;

pub trait Special{
    fn parse(&self, Node: &mut Node<UINode>, UIData: &res_UIData);
}