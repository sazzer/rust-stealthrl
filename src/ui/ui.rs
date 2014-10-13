extern crate ncurses;

#[doc = "
The actual representation of the User Interface
"]
#[experimental]
pub struct UI;

impl Drop for UI {
    #[experimental]
    fn drop(&mut self) {
        ncurses::getch();
        debug!("Destroying the User Interface");
        ncurses::endwin();
    }
}

impl UI {
    #[doc = "
    Get the width of the user interface, in cells
    
    # Return value
    The number of cells wide the user interface can be
    "]
    #[experimental]
    pub fn width(&self) -> uint {
        ncurses::COLS as uint
    }
    #[doc = "
    Get the height of the user interface, in cells
    
    # Return value
    The number of cells tall the user interface can be
    "]
    #[experimental]
    pub fn height(&self) -> uint {
        ncurses::LINES as uint
    }
}

#[doc = "
Create the user interface.

# Return value
The actual user interface
"]
#[experimental]
pub fn create_ui() -> UI {
    debug!("Creating the User Interface");
    ncurses::initscr();
    UI
}