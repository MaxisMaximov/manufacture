use super::*;

pub trait UI_Special{
    fn render(&self, IN_UIData: &res_UIData) -> String;
}

pub fn progressBar(IN_length: usize, IN_val: usize, IN_maxVal: usize) -> String{
    let w_filledIn = (IN_length * ((IN_val * 100) / IN_maxVal)) / 100; // (LEN * PERCENT) / 100
    " ".repeat(IN_length) // Empty progressbar
        .replacen(" ", "█", w_filledIn) // Replace
}
pub fn percent(IN_val: usize, IN_maxVal: usize) -> String{
    format!("{}%", ((IN_val * 100) / IN_maxVal))
}
pub fn list(IN_items: &[String], IN_maxWidth: usize, IN_vertical: bool) -> String{
    let mut OUT_str = String::new();

    for ITEM in IN_items.iter(){
        OUT_str.push_str(&ITEM[0..IN_maxWidth]);
        if IN_vertical{
            OUT_str.push('\n'); // Newline append
        }
        else{
            OUT_str.push_str(" | ");
        }
    }

    OUT_str
}