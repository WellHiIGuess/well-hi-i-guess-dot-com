use crate::{library::{page::page}, elements::hello::Hello};

pub fn home() -> String {
    let count = 10;
    page(vec![
        &Hello {}; count
    ])
}
