extern crate jsonpath;
extern crate serde_json;

use jsonpath::Selector;
use serde_json::Value;

const JSONDOC: &'static str = r#"
    { "store": {
        "books": [
          { "category": "reference",
            "author": "Nigel Rees",
            "title": "Sayings of the Century",
            "price": 8.95
          },
          { "category": "fiction",
            "author": "Evelyn Waugh",
            "title": "Sword of Honour",
            "price": 12.99
          },
          { "category": "fiction",
            "author": "Herman Melville",
            "title": "Moby Dick",
            "isbn": "0-553-21311-3",
            "price": 8.99
          },
          { "category": "fiction",
            "author": "J. R. R. Tolkien",
            "title": "The Lord of the Rings",
            "isbn": "0-395-19395-8",
            "price": 22.99
          }
        ],
        "bicycle": {
          "color": "red",
          "price": 19.95
        }
      }
    }
"#;

macro_rules! assert_jsonpath_f64 {
    ($path:expr, $expected:expr) => {
        assert_jsonpath!(JSONDOC, $path, f64, as_f64, $expected);
    }
}

macro_rules! assert_jsonpath_str {
    ($path:expr, $expected:expr) => {
        assert_jsonpath!(JSONDOC, $path, &str, as_str, $expected);
    }
}

macro_rules! assert_jsonpath {
    ($json:expr, $path:expr, $type:ty, $convert:ident, $expected:expr) => {
        let value: Value = serde_json::from_str($json).unwrap();
        let selector = Selector::new($path).unwrap();
        let selected_values: Vec<$type> = selector.find(&value).map(|x| x.$convert().unwrap()).collect();
        assert_eq!(selected_values, $expected);
    }
}


#[test]
fn test_find() {
    assert_jsonpath_f64!("$.store.bicycle.price", [19.95]);
}

#[test]
fn test_index() {
    assert_jsonpath_str!("$.store.books[2].title", ["Moby Dick"]);
}

#[test]
fn test_slice() {
    assert_jsonpath_f64!("$.store.books[1:2].price", [12.99, 8.99]);
}

#[test]
fn test_slice_to() {
    assert_jsonpath_f64!("$.store.books[:3].price", [8.95, 12.99, 8.99]);
}

#[test]
fn test_slice_from() {
    assert_jsonpath_f64!("$.store.books[1:].price", [12.99, 8.99, 22.99]);
}

#[test]
fn test_root() {
    let json = r#"
        {"a": 10}
    "#;
    let value: Value = serde_json::from_str(json).unwrap();
    let selector = Selector::new("$").unwrap();
    let _found_values: Vec<&Value> = selector.find(&value).collect();
}
