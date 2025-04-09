use super::*;

pub mod specials;

pub struct UIElement{
    pub tag: UITag,
    pub style: UIStyle
}

pub struct UICalculated{
    pub offset: (usize, usize),
    pub box_bounding: (usize, usize)
}

pub enum UITag{
    None,
    /// You *can* add subnodes to this one  
    /// But it's recommended not to as it can break stuff
    Text(String),
    /// # WARNING
    /// The node **MUST NOT HAVE** subnodes  
    /// Specials fully reconstruct their nodes
    Special(Box<dyn specials::UISpecial>)
}
impl UITag{
    pub fn take(&mut self) -> UITag{
        std::mem::replace(self, UITag::None)
    }
    /// Yes, this is stupid  
    /// But I like it lol
    pub fn giveBack(&mut self, IN_val: Self){
        let _ = std::mem::replace(self, IN_val);
    }
}

pub struct UIStyle{
    pub position: UIPos,
    pub size: UISize,
    pub fg: Color,
    pub bg: Color,
    pub border: UIBorder,
    pub display: UIDisplay
}

pub enum UIPos{
    Static,
    Abs(Vector2),
    Rel(Vector2)
}
pub enum UISize{
    Abs((usize, usize)),
    Frac((usize, usize))
}
pub enum UIBorder{
    None,
    SingleChar(char),
    Fancy
}pub enum UIDisplay{
    Float
}