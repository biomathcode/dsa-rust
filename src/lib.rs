pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod problems;
pub use problems::*;

pub mod ds;
pub use ds::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
