use crossterm::terminal::enable_raw_mode;

// SYS_[...]        -- essentials
// TEMPLATE_[...]   -- templates for stuff
// GAME_[...]       -- actuall objects
// x_[...]          -- struct functions, x is first letter of struct name
// [...]_[...]      -- local variables, first [...] is function name in CAPS
// INx_[...]        -- input variables for functions, x is first letter of struct name

fn main() {
    enable_raw_mode().unwrap();
    let mut SYS_GAME_START = manufacture::manufacture::SYS_GAME::new();
    SYS_GAME_START.GAME_loop()
}