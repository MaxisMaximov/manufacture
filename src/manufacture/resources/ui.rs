use super::*;

pub struct UI_element{
    pub position: Vector2,
    pub content: String,
    pub request: Option<&'static str>,
    pub fg: Color,
    pub bg: Color
}