extern crate stealthrlui;

use stealthrlui::ui;

#[cfg(not(test))]
fn main() {
    ui::ui();
}
