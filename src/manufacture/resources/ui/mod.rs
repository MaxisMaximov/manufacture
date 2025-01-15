use super::*;

pub mod specials;

pub struct UI_element{
    pub tag: UI_tag,
    pub style: UI_style
}
pub struct UI_parentData{
    pub position: Vector2,
    pub size: (usize, usize)
}
impl UI_parentData{
    pub fn concatStyle(&self, IN_style: &UI_style) -> UI_parentData{
        Self{
            position: {
                match IN_style.position {
                    UI_pos::Static => self.position,
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

pub enum UI_tag{
    none,
    /// You *can* add subnodes to this one  
    /// But it's recommended not to as it can break stuff
    text(String),
    /// # WARNING
    /// The node **MUST NOT HAVE** subnodes  
    /// Specials fully reconstruct their nodes
    special(Box<dyn specials::UI_Special>)
}
impl UI_tag{
    pub fn take(&mut self) -> UI_tag{
        std::mem::replace(self, UI_tag::none)
    }
    /// Yes, this is stupid  
    /// But I like it lol
    pub fn giveBack(&mut self, IN_val: Self){
        let _ = std::mem::replace(self, IN_val);
    }
}

pub struct UI_style{
    pub position: UI_pos,
    pub size: UI_size,
    pub fg: Color,
    pub bg: Color,
    pub border: UI_border,
    pub display: UI_display
}

pub enum UI_pos{
    Static,
    Abs(Vector2),
    Rel(Vector2)
}
pub enum UI_size{
    Abs((usize, usize)),
    Frac((usize, usize))
}
pub enum UI_border{
    None,
    SingleChar(char),
    Fancy
}pub enum UI_display{
    Float
}