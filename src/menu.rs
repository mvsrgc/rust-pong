enum MenuChoices {
    Resume = 0,
    ToggleMusic = 1,
    Quit = 2,
    ResetGame = 3,
    ItemsTotal = 4,
}

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
            self.current_menu_choice += MenuChoices::ItemsTotal as isize;
        }

        if self.current_menu_choice >= MenuChoices::ItemsTotal as isize {
            self.current_menu_choice -= MenuChoices::ItemsTotal as isize;
        }
    }
}
