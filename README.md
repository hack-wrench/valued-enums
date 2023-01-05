# valued-enums
Macros collection and specify trait for creating valued or python-like enums.

# Installation
```toml
[dependencies.valued-enums]
version = "1.0.0"
#git = "https://github.com/hack-wrench/valued-enums"
```

# Example
When you writing your enums collection, i recommend using `pub` to expand the trait in the rest of your project.
```rust
pub use valued_enums::*;


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

fn main() {
    println!("Get key: {}", RustLikeEnum::ONE.key());
    println!("Get value: {}", PyLikeEnum::TWO.value());
    
    println!("Get all keys: {:?}", RustLikeEnum::keys());
    println!("Get all values: {:?}", PyLikeEnum::values());

    println!("Get all variants: {:?}", some_inner_module::VisibleCustomizeEnum::variants());

    // It can be dangerous
    println!("Convert title to enum: {}", some_inner_module::VisibleCustomizeEnum::from_key("PRIVATE").unwrap().value());
}
```
