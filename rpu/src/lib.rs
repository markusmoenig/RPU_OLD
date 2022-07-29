mod rpuc;
pub mod rpu;

pub use crate::rpu::RPU as RPU;

#[macro_use]
extern crate alloc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
