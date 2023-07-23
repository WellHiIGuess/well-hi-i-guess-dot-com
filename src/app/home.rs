use std::cell::RefCell;

use crate::library::{element::Element, page::Page};

pub struct Home {
    elements: Box<RefCell<Vec<&'static mut dyn Element>>>,
}

impl Home {
    pub fn new() -> Self {
        Self {
            elements: Box::new(RefCell::new(vec![

            ])),
        }
    }
}

impl Page for Home {
    fn return_elements(&mut self) -> Option<&Box<RefCell<Vec<&'static mut dyn Element>>>> {
        Some(&self.elements)
    }
}
