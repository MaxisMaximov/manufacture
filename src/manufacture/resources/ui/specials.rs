use super::*;

pub trait UI_Special{
    fn render(&self, IN_UIData: &res_UIData) -> String;
}

pub struct UISpec_progressBar{
    pub length: usize,
    pub maxVal: usize,
    pub trackVal: String
}
impl UI_Special for UISpec_progressBar{
    fn render(&self, IN_UIData: &res_UIData) -> String {

        let idkfa_val =
            IN_UIData.get(&self.trackVal)
                .expect(&format!("ERROR: Value {} for ProgressBar Special does not exist", self.trackVal))
            .downcast_ref::<usize>()
                .expect(&format!("ERROR: Value {} for ProgressBar Special is not a usize", self.trackVal));

        let w_filledIn = (self.length * ((idkfa_val * 100) / self.maxVal)) / 100; // (LEN * PERCENT) / 100

        " ".repeat(self.length)
            .replacen(" ", "█", w_filledIn)
    }
}

pub struct UISpec_percent{
    pub maxVal: usize,
    pub trackVal: String
}
impl UI_Special for UISpec_percent{
    fn render(&self, IN_UIData: &res_UIData) -> String {
        let idkfa_val =
            IN_UIData.get(&self.trackVal)
                .expect(&format!("ERROR: Value {} for Percent Special does not exist", self.trackVal))
            .downcast_ref::<usize>()
                .expect(&format!("ERROR: Value {} for Percent Special is not a usize", self.trackVal));

        format!("{}%", ((idkfa_val * 100) / self.maxVal))
    }
}

pub struct UISpec_list{
    pub maxItemSize: usize,
    pub vertical: bool,
    pub trackVal: String
}
impl UI_Special for UISpec_list{
    fn render(&self, IN_UIData: &res_UIData) -> String {
        let idkfa_val =
            IN_UIData.get(&self.trackVal)
                .expect(&format!("ERROR: Value {} for List Special does not exist", self.trackVal))
            .downcast_ref::<Vec<String>>()
                .expect(&format!("ERROR: Value {} for List Special is not a String Vector", self.trackVal));
        
        let mut OUT_str = String::new();

        for ITEM in idkfa_val.iter(){

            OUT_str.push_str(&ITEM[0..self.maxItemSize]);

            if self.vertical{
                OUT_str.push('\n'); // Newline append
            }
            else{
                OUT_str.push_str(" | ");
            }
        }

        OUT_str
    }
}