use std::collections::HashMap;

// Custom Error Types
#[derive(Debug)]
pub enum Error {
    ParseConfigFailure,
    RedefinitionOfShorthand,
}

pub struct Translator {
    shorthand: HashMap<String, String>,
}
impl Translator {
    pub fn from(config_text: &str) -> Result<Self, Error> {
        
        let shorthand = Self::extract_hashmap_from_config(config_text.to_string());
        Ok(Self { shorthand })
    }

    fn extract_hashmap_from_config(config_text: String) -> HashMap<String, String> {
        let mut shorthand = HashMap::new();
        let defs = config_text.split(',');
        for def in defs {
            let pieces = def.trim().split("=>").collect::<Vec<_>>();
            for arr in pieces.chunks(2) {
                shorthand.insert(arr[0].trim().to_string(), arr[1].trim().to_string());
            }
        }
        shorthand
    }

    pub fn translate(&self, msg: &str) -> String {
        let mut result = String::from(msg);
        for (old, new) in self.shorthand.iter() {
            result = result.replace(old, new);
        }
        result
    }
}

// Tests | This is how I want it to work

mod tests {

    use super::Translator;
    use std::collections::HashMap;

    #[test]
    fn translator_api_requirements() {
        let my_tran =
            Translator::from("x =>shorthand, y => roll tide,z=> another              ").unwrap();
        let mut test_map = HashMap::new();
        test_map.insert("x".to_string(), "shorthand".to_string());
        test_map.insert("y".to_string(), "roll tide".to_string());
        test_map.insert("z".to_string(), "another".to_string());
        assert_eq!(my_tran.shorthand, test_map);

        let result = my_tran.translate("x y z");
        assert_eq!("shorthand roll tide another".to_string(), result);
    }
}
