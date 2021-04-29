mod type_list;
mod list;
#[macro_use]
mod macros;
mod demo;


#[cfg(test)]
mod tests {
    fn my_true(i: usize) -> bool {
        true
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
