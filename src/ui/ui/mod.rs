#[doc = "The actual User Interface"]
pub struct UI;

impl Drop for UI {
    #[stable]
    #[doc = "
    Tidy up everything about the UI when it closes
    "]
    fn drop(&mut self) {
        debug!("Destroying UI");
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
    UI
}