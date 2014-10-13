#![feature(phase)]
#[phase(plugin, link)] extern crate log;

extern crate stealthrlui;

use stealthrlui::ui;

#[cfg(not(test))]
fn main() {
    let ui = ui::create_ui();
    debug!("This is after the UI is created");
    debug!("UI size is {}x{}", ui.width(), ui.height());
}
