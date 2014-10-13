#![feature(phase)]
#[phase(plugin, link)] extern crate log;

pub mod ui {
    extern crate ncurses;
    
    #[doc = "
    The actual representation of the User Interface
    "]
    pub struct UI;
    
    impl Drop for UI {
        fn drop(&mut self) {
            debug!("Destroying the User Interface");
        }
    }
    
    
    #[doc = "
    Create the user interface.
    
    # Return value
        The actual user interface
    "]
    pub fn create_ui() -> UI {
        debug!("Creating the User Interface");
        UI
    }
    
    pub fn ui() {
        ncurses::initscr();
        ncurses::printw("Hello, World");
        ncurses::refresh();
        ncurses::getch();
        ncurses::endwin();
    }
}