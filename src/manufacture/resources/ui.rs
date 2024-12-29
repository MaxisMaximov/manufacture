use super::*;

pub struct UI_element{
    pub position: UI_pos,
    pub content: String,
    pub fg: Color,
    pub bg: Color,
    pub border: UI_border,
    pub borderSize: (usize, usize) // Inner size
}
pub struct UI_data{
    pub position: Vector2
}
impl UI_data{
    pub fn concat(&self, IN_data: &UI_data) -> UI_data{
        UI_data{
            position: (self.position.0 + IN_data.position.0, self.position.1 + IN_data.position.1),
        }
    }
}

pub enum UI_pos{
    Abs(Vector2),
    Rel(Vector2)
}
pub enum UI_border{
    singleChar(char)
}