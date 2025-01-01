use super::*;

pub struct UI_element{
    pub content: fn(&res_UIData) -> String,
    pub style: UI_style
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
    pub fn concatStyle(&self, IN_style: &UI_style) -> UI_data{
        Self{
            position: {
                match IN_style.position {
                    UI_pos::Abs(POS) => (self.position.0 + POS.0, self.position.1 + POS.1),
                    UI_pos::Rel(POS) => POS,
                }
            },
        }
    }
}

pub struct UI_style{
    pub position: UI_pos,
    pub fg: Color,
    pub bg: Color,
    pub border: UI_border,
    pub size: (usize, usize)
}

pub enum UI_pos{
    Abs(Vector2),
    Rel(Vector2)
}
pub enum UI_border{
    singleChar(char)
}
