use super::*;

pub static widgetsMap: Lazy<Mutex<widgets_master>> = Lazy::new(|| Mutex::new(widgets_master::new()));

pub struct widgets_master{
    pub textBoxes: Vec<textBox>
}
impl widgets_master{
    pub fn new() -> Self{
        Self{
            textBoxes: Vec::new()
        }
    }
    pub fn cleanup_textBoxes(&mut self){
        self.textBoxes.retain(|x| !x.markForDel)
    }
}

/// # Position in Render Buffer
/// A selection of common positions for useage
///
/// You can also use your custom position with `POS_custom`
#[allow(unused)]
pub enum position {
    middle,
    right,
    left,
    top,
    bottom,
    TL,
    TR,
    BL,
    BR,
    custom(types::vector2),
}
impl position {
    pub fn value(&self) -> types::vector2 {
        match *self {
            Self::middle => (vars::RENDERER::RENDER_BUFFER_X / 2, vars::RENDERER::RENDER_BUFFER_Y / 2),
            Self::left => (0, vars::RENDERER::RENDER_BUFFER_Y / 2),
            Self::right => (vars::RENDERER::RENDER_BUFFER_X - 1, vars::RENDERER::RENDER_BUFFER_Y / 2),
            Self::top => (vars::RENDERER::RENDER_BUFFER_X / 2, 0),
            Self::bottom => (vars::RENDERER::RENDER_BUFFER_X / 2, vars::RENDERER::RENDER_BUFFER_Y - 1),
            Self::TL => (0, 0),
            Self::TR => (vars::RENDERER::RENDER_BUFFER_X - 1, 0),
            Self::BL => (0, vars::RENDERER::RENDER_BUFFER_Y - 1),
            Self::BR => (vars::RENDERER::RENDER_BUFFER_X - 1, vars::RENDERER::RENDER_BUFFER_Y - 1),
            Self::custom(POS) => POS,
        }
    }
}

pub struct textBox {
    pub position: self::position,
    pub string: String,
    pub values: String,
    pub lifetime: u16,
    pub markForDel: bool,
}
impl textBox {
    pub fn new(
        IN_pos: self::position,
        IN_text: &str,
        IN_values: &str,
        IN_lifetime: u16,
    ) -> Self {
        Self {
            position: IN_pos,
            string: IN_text.to_owned(),
            values: IN_values.to_owned(),
            lifetime: IN_lifetime,
            markForDel: false,
        }
    }

    pub fn tickdown(&mut self) {
        // If it's marked for del just ignore
        if self.markForDel {
            return;
        }
        // If it's ""permament"" then don't do anything
        if self.lifetime == 255 {
            return;
        }
        // If lifetime is 0, mark for deletion
        if self.lifetime == 0 {
            self.markForDel = true;
            return;
        }
        self.lifetime -= 1;
    }
}