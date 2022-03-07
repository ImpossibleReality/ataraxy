pub mod framework;

pub use ataraxy_macros::*;
pub use framework::Command;
pub use framework::Context;
pub use framework::Framework;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
