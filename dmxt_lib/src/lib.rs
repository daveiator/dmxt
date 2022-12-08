pub mod dmx;
pub mod threads;
pub mod timing;
pub mod builders;
pub mod macros;
pub mod components;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}