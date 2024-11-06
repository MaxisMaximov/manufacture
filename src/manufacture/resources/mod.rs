use super::*;

pub struct res_PInput{
    res: KeyEvent
}
impl gmRes for res_PInput{
    fn new() -> Self {
        Self{
            res: KeyEvent::new(KeyCode::Null, KeyModifiers::NONE)
        }
    }

    fn RES_ID() -> &'static str {
        "res_PInput"
    }
}

pub struct res_DeltaT{
    res: Duration
}
impl gmRes for res_DeltaT{
    fn new() -> Self {
        Self{
            res: Duration::from_secs(0)
        }
    }

    fn RES_ID() -> &'static str {
        "res_DeltaT"
    }
}

pub struct res_Event{
    res: Vec<&'static str>
}
impl gmRes for res_Event{
    fn new() -> Self {
        Self{
            res: Vec::new()
        }
    }

    fn RES_ID() -> &'static str {
        "res_Event"
    }
}