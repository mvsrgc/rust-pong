enum MenuChoices {
    Resume = 0,
    ToggleSounds = 1,
    ToggleParticles = 2,
    Quit = 3,
    ResetGame = 4,
    ItemsTotal = 5,
}

impl MenuChoices {
    fn items_total() -> isize {
        MenuChoices::ItemsTotal as isize
    }
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
            self.current_menu_choice += MenuChoices::items_total();
        }

        if self.current_menu_choice >= MenuChoices::items_total() {
            self.current_menu_choice -= MenuChoices::items_total();
        }
    }
}
