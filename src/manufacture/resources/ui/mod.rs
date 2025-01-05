use super::*;

pub mod specials;

pub struct UI_element{
    pub content: fn(&res_UIData) -> String,
    pub style: UI_style
}
pub struct UI_data{
    pub position: Vector2,
    pub size: (usize, usize)
}
impl UI_data{
    pub fn concatStyle(&self, IN_style: &UI_style) -> UI_data{
        Self{
            position: {
                match IN_style.position {
                    UI_pos::Abs(POS) => POS,
                    UI_pos::Rel(POS) => (self.position.0 + POS.0, self.position.1 + POS.1),
                }
            },
            size: {
                match IN_style.size{
                    UI_size::Abs(SIZE) => SIZE,
                    UI_size::Frac(FRAC) => ((self.size.0 * FRAC.0) / 100, (self.size.1 * FRAC.1) / 100),
                }
            }
        }
    }
}

pub struct UI_style{
    pub position: UI_pos,
    pub size: UI_size,
    pub fg: Color,
    pub bg: Color,
    pub border: UI_border,
}

pub enum UI_pos{
    Abs(Vector2),
    Rel(Vector2)
}
pub enum UI_size<T = (usize, usize)>{ // A generic to not rewrite constantly lol
    Abs(T),
    Frac(T)
}
pub enum UI_border{
    none,
    singleChar(char),
    fancy
}