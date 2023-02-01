#![feature(proc_macro_hygiene, custom_inner_attributes)]
use macro_module::add_answer;

mod a;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_adds_answer() {
        assert_eq!(a::answer(), 42)
    }

    #[test]
    fn macro_does_not_violate_original() {
        assert_eq!(a::original(), 42)
    }
}
