extern crate ncurses;

#[doc = "The actual User Interface"]
pub struct UI;

impl Drop for UI {
    #[stable]
    #[doc = "
    Tidy up everything about the UI when it closes
    "]
    fn drop(&mut self) {
        ncurses::getch();
        debug!("Destroying UI");
        ncurses::endwin();
    }
}

impl UI {
    #[experimental]
    #[doc = "
    Get the width of the user interface in cells
    
    # Returns
    The number of cells wide
    "]
    pub fn width(&self) -> uint {
        ncurses::COLS as uint
    }
    #[experimental]
    #[doc = "
    Get the heigth of the user interface in cells
    
    # Returns
    The number of cells tall
    "]
    pub fn height(&self) -> uint {
        ncurses::LINES as uint
    }
}
#[experimental]
#[doc = "
Actually create the User Interface

# Returns
The object representing the user interface
"]
pub fn create_ui() -> UI {
    debug!("Creating UI");
    ncurses::initscr();
    UI
}