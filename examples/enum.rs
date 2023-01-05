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

fn main() {
    println!("Get key: {}", RustLikeEnum::ONE.key());
    println!("Get value: {}", PyLikeEnum::TWO.value());
    
    println!("Get all keys: {:?}", RustLikeEnum::keys());
    println!("Get all values: {:?}", PyLikeEnum::values());

    println!("Get all variants: {:?}", some_inner_module::VisibleCustomizeEnum::variants());

    // It can be dangerous
    println!("Convert title to enum: {}", some_inner_module::VisibleCustomizeEnum::from_key("PRIVATE").unwrap().value());
}
