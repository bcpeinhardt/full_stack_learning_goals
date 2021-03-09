//! The translating code for my api
//!

use std::collections::HashMap;

type Config = HashMap<&'static str, &'static str>;

#[derive(Debug)]
enum Error {
    _placeholder,
}

fn translate(dictionary: Config, msg: &str) -> Option<String> {
    if let Some(&val) = dictionary.get(msg) {
        Some(val.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn basic_translate() {
        use super::*;

        let mut dictionary = HashMap::new();
        dictionary.insert("Roll Tide", "Holy Hot Hell Roll Motha Flippin Tide");
        dictionary.insert("hehe", "haha");

        assert_eq!("haha", translate(dictionary.clone(), "hehe").unwrap());

        assert_eq!(
            "Holy Hot Hell Roll Motha Flippin Tide",
            translate(dictionary.clone(), "Roll Tide").unwrap()
        );
    }
}
