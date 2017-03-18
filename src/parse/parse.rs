pub fn tib() -> &'static str {
    let foo = "\u{f40}";
    foo
}

#[cfg(test)]
mod tests {
    use super::tib;

    #[test]
    fn it_works() {
        assert_eq!("ཀ", tib());
    }
}
