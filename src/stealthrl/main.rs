extern crate ncurses;
#[cfg(not(test))]
fn main() {
    ncurses::initscr();
    ncurses::printw("Hello, World");
    ncurses::refresh();
    ncurses::getch();
    ncurses::endwin();
}
