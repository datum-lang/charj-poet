// todo: remove dead code, it's temp solution for better debug
#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

extern crate serde;

pub mod poet;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
