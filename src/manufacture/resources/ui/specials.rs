use super::*;

pub trait UISpecial{
    fn parse(&self, IN_node: &mut Node<UIElement>, IN_resUIData: &res_UIData);
}

impl Node<UIElement>{
    pub fn UI_ERROR(&mut self, IN_errorStr: String){
        let idkfa_len = IN_errorStr.len();
        self.add_node(
            UIElement{
                tag: UITag::Text(IN_errorStr),
                style: UIStyle{
                    position: UIPos::Static,
                    size: UISize::Abs((idkfa_len, 1)),
                    fg: Color::White,
                    bg: Color::Red,
                    border: UIBorder::Fancy,
                    display: UIDisplay::Float
                },
            }
        );
    }
}

pub struct UISpecProgressBar{
    pub length: usize,
    pub max_val: usize,
    pub track_val: String
}
impl UISpecial for UISpecProgressBar{
    fn parse(&self, IN_node: &mut Node<UIElement>, IN_resUIData: &res_UIData){

        let w_val = match IN_resUIData.get(&self.track_val){
            Some(BOX) => {
                match BOX.downcast_ref::<usize>(){
                    Some(VAL) => VAL,
                    None => return IN_node.UI_ERROR(format!("ERROR: Value {} for ProgressBar Special is not a usize", self.track_val))
                }
            }, // I can just Return with the function as the return type is null in both lol
            None => return IN_node.UI_ERROR(format!("ERROR: Value {} for ProgressBar Special does not exist", self.track_val))
        };

        let w_filledIn = (self.length * ((w_val * 100) / self.max_val)) / 100; // (LEN * PERCENT) / 100

        IN_node.add_node(
            UIElement{
                tag: UITag::Text(" ".repeat(self.length).replacen(" ", "█", w_filledIn)),
                style: UIStyle{
                    position: UIPos::Static,
                    size: UISize::Abs((self.length, 1)),
                    fg: Color::White,
                    bg: Color::Black,
                    border: UIBorder::None,
                    display: UIDisplay::Float
                }});
    }
}

pub struct UISpecPercent{
    pub max_val: usize,
    pub track_val: String
}
impl UISpecial for UISpecPercent{
    fn parse(&self, IN_node: &mut Node<UIElement>, IN_resUIData: &res_UIData){

        let w_val = match IN_resUIData.get(&self.track_val){
            Some(BOX) => {
                match BOX.downcast_ref::<usize>(){
                    Some(VAL) => VAL,
                    None => return IN_node.UI_ERROR(format!("ERROR: Value {} for Percent Special is not a usize", self.track_val))
                }
            },
            None => return IN_node.UI_ERROR(format!("ERROR: Value {} for Percent Special does not exist", self.track_val)),
        };

        let idkfa_str = format!("{}%", ((w_val * 100) / self.max_val));
        let idkfa_len = idkfa_str.len();

        IN_node.add_node(
            UIElement{
                tag: UITag::Text(idkfa_str),
                style: UIStyle{
                    position: UIPos::Rel((0, 0)),
                    size: UISize::Abs((idkfa_len, 1)), // +1 X is the % symbol
                    fg: Color::White,
                    bg: Color::Black,
                    border: UIBorder::None,
                    display: UIDisplay::Float
                }});
    }
}

pub struct UISpecList{
    pub max_item_size: usize,
    pub vertical: bool,
    pub track_val: String
}
impl UISpecial for UISpecList{
    fn parse(&self, IN_node: &mut Node<UIElement>, IN_resUIData: &res_UIData){

        let w_val = match IN_resUIData.get(&self.track_val){
            Some(BOX) => {
                match BOX.downcast_ref::<Vec<String>>(){
                    Some(VAL) => VAL,
                    None => return IN_node.UI_ERROR(format!("ERROR: Value {} for List Special is not a String Vector", self.track_val))
                }
            },
            None => return IN_node.UI_ERROR(format!("ERROR: Value {} for List Special does not exist", self.track_val)),
        };

        IN_node.map(|parent| {
            let mut w_pos: Vector2;
            let mut w_content: String;

            for (INDEX, ITEM) in w_val.iter().enumerate(){
                if self.vertical{
                    w_pos = (0, -(INDEX as isize));
                    w_content = ITEM.clone();
                }else{
                    w_pos = (((self.max_item_size + 3) * INDEX) as isize, 0);
                    w_content = ITEM.clone() + " | ";
                }
                let _ = parent.add_node(
                    UIElement{
                        tag: UITag::Text(w_content),
                        style: UIStyle{
                            position: UIPos::Rel(w_pos),
                            size: UISize::Abs((self.max_item_size, 1)),
                            fg: Color::White,
                            bg: Color::Black,
                            border: UIBorder::None,
                            display: UIDisplay::Float
                        },
                    }
                );
            }
        });
    }
}