use super::*;

pub trait gmRes: Any{
    fn new() -> Self;
    fn RES_ID() -> &'static str;
}