pub mod core;
pub mod auxiliary;
pub mod serde;
pub mod manipulator;
pub mod plugin;
pub(crate) mod strings;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
