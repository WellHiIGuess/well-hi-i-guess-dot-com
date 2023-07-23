use std::cell::RefCell;

use super::element::Element;

pub trait Page {
    fn return_elements(&mut self) -> Option<&Box<RefCell<Vec<&'static mut dyn Element>>>>;
}
