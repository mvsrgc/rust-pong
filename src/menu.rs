pub const TOTAL_ITEMS: isize = 5;

pub struct Menu {
    pub current_menu_choice: isize,
}

impl Menu {
    pub fn new(current_menu_choice: isize) -> Menu {
        Menu {
            current_menu_choice,
        }
    }

    pub fn advance_menu_choice(&mut self, delta: isize) {
        self.current_menu_choice -= delta;

        if self.current_menu_choice < 0 {
            self.current_menu_choice += TOTAL_ITEMS;
        }

        if self.current_menu_choice >= TOTAL_ITEMS {
            self.current_menu_choice -= TOTAL_ITEMS;
        }
    }
}
