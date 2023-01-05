use crate::*;


py_enum! {
    #[derive(Debug)]
    PyLikeEnum(u8):
        ONE = 1
        TWO = 2
        THREE = 3
}

rust_enum! {
    #[derive(Debug)]
    enum RustLikeEnum((u8, u8)) {
        ONE = (1, 1),
        TWO = (2, 2),
        THREE = (3, 3),
    }
}

valued_enum! {
    #[derive(Debug)]
    pub enum FullCustomizeRustLike(&'static str) {
        pub(crate) CRATE_VISIBLE = "crate",
        pub PUBLIC_VISIBLE = "public",
        PRIVATE = "private",
    }
}

#[test]
fn enums() {
    assert_eq!(PyLikeEnum::ONE, PyLikeEnum::ONE);
    assert_eq!(PyLikeEnum::TWO.value(), 2);

    assert_eq!(RustLikeEnum::THREE.key(), "THREE");
    assert_eq!(RustLikeEnum::values(), vec![
        (1, 1), (2, 2), (3, 3),
    ]);

    assert_eq!(FullCustomizeRustLike::PRIVATE.value(), "private")
}
