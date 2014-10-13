pub mod ui {
    extern crate ncurses;
    
    pub fn ui() {
        ncurses::initscr();
        ncurses::printw("Hello, World");
        ncurses::refresh();
        ncurses::getch();
        ncurses::endwin();
    }
}