use super::*;

pub trait UI_Special{
    fn parse(&self, IN_node: &mut Node<UI_element>, IN_resUIData: &res_UIData);
}

impl Node<UI_element>{
    pub fn UI_ERROR(&mut self, IN_errorStr: String){
        let idkfa_len = IN_errorStr.len();
        self.addNode(
            UI_element{
                tag: UI_tag::text(IN_errorStr),
                style: UI_style{
                    position: UI_pos::Rel((0, 0)),
                    size: UI_size::Abs((idkfa_len, 1)),
                    fg: Color::White,
                    bg: Color::Red,
                    border: UI_border::fancy,
                },
            }
        );
    }
}

pub struct UISpec_progressBar{
    pub length: usize,
    pub maxVal: usize,
    pub trackVal: String
}
impl UI_Special for UISpec_progressBar{
    fn parse(&self, IN_node: &mut Node<UI_element>, IN_resUIData: &res_UIData){

        let w_val = match IN_resUIData.get(&self.trackVal){
            Some(BOX) => {
                match BOX.downcast_ref::<usize>(){
                    Some(VAL) => VAL,
                    None => return IN_node.UI_ERROR(format!("ERROR: Value {} for ProgressBar Special is not a usize", self.trackVal))
                }
            }, // I can just Return with the function as the return type is null in both lol
            None => return IN_node.UI_ERROR(format!("ERROR: Value {} for ProgressBar Special does not exist", self.trackVal))
        };

        let w_filledIn = (self.length * ((w_val * 100) / self.maxVal)) / 100; // (LEN * PERCENT) / 100

        IN_node.addNode(
            UI_element{
                tag: UI_tag::text(" ".repeat(self.length).replacen(" ", "█", w_filledIn)),
                style: UI_style{
                    position: UI_pos::Rel((0, 0)),
                    size: UI_size::Abs((self.length, 1)),
                    fg: Color::White,
                    bg: Color::Black,
                    border: UI_border::none,
                }});
    }
}

pub struct UISpec_percent{
    pub maxVal: usize,
    pub trackVal: String
}
impl UI_Special for UISpec_percent{
    fn parse(&self, IN_node: &mut Node<UI_element>, IN_resUIData: &res_UIData){

        let w_val = match IN_resUIData.get(&self.trackVal){
            Some(BOX) => {
                match BOX.downcast_ref::<usize>(){
                    Some(VAL) => VAL,
                    None => return IN_node.UI_ERROR(format!("ERROR: Value {} for Percent Special is not a usize", self.trackVal))
                }
            },
            None => return IN_node.UI_ERROR(format!("ERROR: Value {} for Percent Special does not exist", self.trackVal)),
        };

        let idkfa_str = format!("{}%", ((w_val * 100) / self.maxVal));
        let idkfa_len = idkfa_str.len();

        IN_node.addNode(
            UI_element{
                tag: UI_tag::text(idkfa_str),
                style: UI_style{
                    position: UI_pos::Rel((0, 0)),
                    size: UI_size::Abs((idkfa_len, 1)), // +1 X is the % symbol
                    fg: Color::White,
                    bg: Color::Black,
                    border: UI_border::none,
                }});
    }
}

pub struct UISpec_list{
    pub maxItemSize: usize,
    pub vertical: bool,
    pub trackVal: String
}
impl UI_Special for UISpec_list{
    fn parse(&self, IN_node: &mut Node<UI_element>, IN_resUIData: &res_UIData){

        let w_val = match IN_resUIData.get(&self.trackVal){
            Some(BOX) => {
                match BOX.downcast_ref::<Vec<String>>(){
                    Some(VAL) => VAL,
                    None => return IN_node.UI_ERROR(format!("ERROR: Value {} for List Special is not a String Vector", self.trackVal))
                }
            },
            None => return IN_node.UI_ERROR(format!("ERROR: Value {} for List Special does not exist", self.trackVal)),
        };

        IN_node.map(|parent| {
            let mut w_pos: Vector2;
            let mut w_content: String;

            for (INDEX, ITEM) in w_val.iter().enumerate(){
                if self.vertical{
                    w_pos = (0, -(INDEX as isize));
                    w_content = ITEM.clone();
                }else{
                    w_pos = (((self.maxItemSize + 3) * INDEX) as isize, 0);
                    w_content = ITEM.clone() + " | ";
                }
                let _ = parent.addNode(
                    UI_element{
                        tag: UI_tag::text(w_content),
                        style: UI_style{
                            position: UI_pos::Rel(w_pos),
                            size: UI_size::Abs((self.maxItemSize, 1)),
                            fg: Color::White,
                            bg: Color::Black,
                            border: UI_border::none,
                        },
                    }
                );
            }
        });
    }
}