# valued-enums
Macros collection and specify trait for creating valued or python-like enums.

# Installation
```toml
[dependencies.valued-enums]
version = "*"
#git = "https://github.com/hack-wrench/valued-enums"
```

# Example
When you writing your enums collection, i recommend using `pub` to expand the trait in the rest of your project.
```rust
use valued_enums::*;


py_enum! {
    PyLikeEnum(u8):
        ONE = 1
        TWO = 2
        THREE = 3
}

rust_enum! {
    enum RustLikeEnum((u8, u8)) {
        ONE = (1, 1),
        TWO = (2, 2),
        THREE = (3, 3),
    }
}

mod some_inner_module {
    use crate::valued_enum;

    valued_enum! {
        #[derive(Debug)]
        pub enum VisibleCustomizeEnum(&'static str) {
            pub(crate) CRATE_VISIBLE = "crate",
            pub PUBLIC_VISIBLE = "public",
            PRIVATE = "private",
        }
    }
}

// To use the match and equal pattern, you must implement #[derive(PartialEq, Eq)]
#[derive(PartialEq, Eq)]
struct Point(i32, i32);

py_enum! {
    PointEnum(Point):
        A = Point(1, 2)
        B = Point(3, 4)
}

fn main() {
    println!("Equal enum with their value type: {}", PyLikeEnum::ONE.equal(&1));

    println!("Get key: {}", RustLikeEnum::ONE.key());
    println!("Get value: {}", PyLikeEnum::TWO.value());

    // May help in the case of a whole enum owned
    println!("Get ref value: &{}", PyLikeEnum::THREE.ref_value());
    
    println!("Get all keys: {:?}", RustLikeEnum::keys());
    println!("Get all values: {:?}", PyLikeEnum::values());

    println!("Get all variants: {:?}", some_inner_module::VisibleCustomizeEnum::variants());

    // Convert to private field can be dangerous!
    println!("Convert from title to enum: {}", some_inner_module::VisibleCustomizeEnum::from_key("PRIVATE").unwrap().value());
    
    // Don't forget to implement for your enum: #[derive(PartialEq, Eq)]
    println!("Convert from value to enum: {}", PyLikeEnum::from_value(&3).unwrap().key());    
}
```
