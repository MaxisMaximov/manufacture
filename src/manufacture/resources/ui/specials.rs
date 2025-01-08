use super::*;

pub trait UI_Special{
    fn parse(&self, IN_resUIData: &res_UIData) -> Node<UI_element>;
}

pub struct UISpec_progressBar{
    pub length: usize,
    pub maxVal: usize,
    pub trackVal: String
}
impl UI_Special for UISpec_progressBar{
    fn parse(&self, IN_resUIData: &res_UIData) -> Node<UI_element> {

        let w_val =
            IN_resUIData.get(&self.trackVal)
                .expect(&format!("ERROR: Value {} for ProgressBar Special does not exist", self.trackVal))
            .downcast_ref::<usize>()
                .expect(&format!("ERROR: Value {} for ProgressBar Special is not a usize", self.trackVal));

        let w_filledIn = (self.length * ((w_val * 100) / self.maxVal)) / 100; // (LEN * PERCENT) / 100

        Node::new(
            UI_element{
                content: UI_content::text(" ".repeat(self.length).replacen(" ", "█", w_filledIn)),
                style: UI_style{
                    position: UI_pos::Rel((0, 0)),
                    size: UI_size::Abs((self.length, 1)),
                    fg: Color::White,
                    bg: Color::Black,
                    border: UI_border::none,
                },
            },
            0,
            0
        )

        
    }
}

pub struct UISpec_percent{
    pub maxVal: usize,
    pub trackVal: String
}
impl UI_Special for UISpec_percent{
    fn parse(&self, IN_resUIData: &res_UIData) -> Node<UI_element> {
        let w_val =
            IN_resUIData.get(&self.trackVal)
                .expect(&format!("ERROR: Value {} for Percent Special does not exist", self.trackVal))
            .downcast_ref::<usize>()
                .expect(&format!("ERROR: Value {} for Percent Special is not a usize", self.trackVal));

        Node::new(
            UI_element{
                content: UI_content::text(format!("{}%", ((w_val * 100) / self.maxVal))),
                style: UI_style{
                    position: UI_pos::Rel((0, 0)),
                    size: UI_size::Abs((self.maxVal + 1, 1)), // +1 is the % symbol
                    fg: Color::White,
                    bg: Color::Black,
                    border: UI_border::none,
                },
            },
            0,
            0
        )

        
    }
}

pub struct UISpec_list{
    pub maxItemSize: usize,
    pub vertical: bool,
    pub trackVal: String
}
impl UI_Special for UISpec_list{
    fn parse(&self, IN_resUIData: &res_UIData) -> Node<UI_element> {
        let w_val =
            IN_resUIData.get(&self.trackVal)
                .expect(&format!("ERROR: Value {} for List Special does not exist", self.trackVal))
            .downcast_ref::<Vec<String>>()
                .expect(&format!("ERROR: Value {} for List Special is not a String Vector", self.trackVal));

        let w_selfSize: (usize, usize) = match self.vertical{
            true => (self.maxItemSize, w_val.len()),
            //                                     There is always n-1 paddings in a horizontal list
            false => ((self.maxItemSize * w_val.len()) + (3 * (w_val.len() - 1)), 1),
        };

        Node::new(
            UI_element{
                content: UI_content::text("".to_owned()),
                style: UI_style{
                    position: UI_pos::Rel((0, 0)),
                    size: UI_size::Abs(w_selfSize),
                    fg: Color::White,
                    bg: Color::Black,
                    border: UI_border::none,
                },
            },
            0,
            1
        ).withNodes(|parent| {
            let mut w_pos: Vector2;
            let mut w_content: String;

            for (INDEX, ITEM) in w_val.iter().enumerate(){
                match self.vertical{
                    true => {
                        w_pos = (0, -(INDEX as isize));
                        w_content = ITEM.clone();
                    },
                    false => {
                        w_pos = (((self.maxItemSize + 3) * INDEX) as isize, 0);
                        w_content = ITEM.clone() + " | ";
                    },
                }
                let _ = parent.addNode(
                    UI_element{
                        content: UI_content::text(w_content),
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
        })
    }
}