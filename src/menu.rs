enum MenuChoices {
    Resume = 0,
    ToggleMusic = 1,
    Quit = 2,
    ItemsTotal = 3,
}

pub struct Menu {
    pub current_menu_choice: isize,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            current_menu_choice: 0,
        }
    }

    pub fn advance_menu_choice(&mut self, delta: isize) {
        self.current_menu_choice -= delta;

        if self.current_menu_choice < 0 {
            self.current_menu_choice += MenuChoices::ItemsTotal as isize;
        }

        if self.current_menu_choice >= MenuChoices::ItemsTotal as isize {
            self.current_menu_choice -= MenuChoices::ItemsTotal as isize;
        }
    }
}
