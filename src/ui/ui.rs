extern crate ncurses;

#[doc = "
The actual representation of the User Interface
"]
pub struct UI;

impl Drop for UI {
    fn drop(&mut self) {
        ncurses::getch();
        debug!("Destroying the User Interface");
        ncurses::endwin();
    }
}


#[doc = "
Create the user interface.

# Return value
    The actual user interface
"]
pub fn create_ui() -> UI {
    debug!("Creating the User Interface");
    ncurses::initscr();
    UI
}