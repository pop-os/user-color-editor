// SPDX-License-Identifier: MPL-2.0-only

use gtk4::{
    glib::{self, subclass::Signal},
    subclass::prelude::*,
    Box, Button, FileChooserNative,
};
use once_cell::sync::Lazy;
use std::{cell::RefCell, rc::Rc};

// Object holding the state
#[derive(Default)]
pub struct ThemeChooserButton {
    pub button: Rc<RefCell<Button>>,
    pub file_chooser: Rc<RefCell<FileChooserNative>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ThemeChooserButton {
    const NAME: &'static str = "ThemeChooserButton";
    type Type = super::ThemeChooserButton;
    type ParentType = Box;
}

// Trait shared by all GObjects
impl ObjectImpl for ThemeChooserButton {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder(
                // Signal name
                "file-selected",
            )
            .build()]
        });
        SIGNALS.as_ref()
    }
}

// Trait shared by all widgets
impl WidgetImpl for ThemeChooserButton {}

// Trait shared by all boxes
impl BoxImpl for ThemeChooserButton {}
