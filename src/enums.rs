pub trait ValuedEnum<T> where Self: Sized {
    fn equal(&self, other: &T) -> bool;

    fn key(&self) -> &str;
    fn value(self) -> T;
    fn ref_value(&self) -> &T;

    fn keys() -> Vec<&'static str>;
    fn values() -> Vec<T>;
    fn variants() -> Vec<Self>;

    fn from_key(key: &str) -> Option<Self>;
    fn from_value(value: &T) -> Option<Self>;
}


#[macro_export]
macro_rules! py_enum {
    {
        $(#[$meta:meta])*
        $name:ident ( $valtype:ty ):
            $(
                $(#[$item_meta:meta])*
                $id:ident = $val:expr
            )*
            $(,)?
    } => {
        $crate::valued_enum! {
            $(#[$meta])*
            pub enum $name ($valtype) {
                $(
                    $(#[$item_meta])*
                    pub $id = $val,
                )*
            }
        }
    }
}

#[macro_export]
macro_rules! rust_enum {
    {
        $(#[$meta:meta])*
        $e_vis:vis enum $name:ident ( $valtype:ty ) {
            $(
                $(#[$item_meta:meta])*
                $id:ident = $val:expr,
            )*
            $(,)?
        }
    } => {
        $crate::valued_enum! {
            $(#[$meta])*
            $e_vis enum $name ($valtype) {
                $(
                    $(#[$item_meta])*
                    pub $id = $val,
                )*
            }
        }
    }
}

#[macro_export]
macro_rules! valued_enum {
    {
       $(#[$meta:meta])*
       $e_vis:vis enum $name:ident ( $valtype:ty ) {
           $(
               $(#[$item_meta:meta])*
               $i_vis:vis $id:ident = $val:expr,
           )*
           $(,)?
       }
    } => {
        $(#[$meta])*
        $e_vis struct $name(&'static str, $valtype);

        impl $name {
            $(
                $(#[$item_meta])*
                $i_vis const $id: $name = $name(stringify!($id), $val);
            )*
        }

        impl $crate::ValuedEnum<$valtype> for $name {
            fn equal(&self, other: &$valtype) -> bool {
                &self.1 == other
            }

            fn key(&self) -> &str {
                self.0
            }

            fn value(self) -> $valtype {
                self.1
            }

            fn ref_value(&self) -> &$valtype {
                &self.1
            }

            fn keys() -> Vec<&'static str> {
                vec![
                    $( $name::$id.key(), )*
                ]
            }

            fn values() -> Vec<$valtype> {
                vec![
                    $( $name::$id.value(), )*
                ]
            }

            fn variants() -> Vec<$name> {
                vec![
                    $( $name::$id, )*
                ]
            }

            fn from_key(key: &str) -> Option<Self> {
                $( if $name::$id.key() == key { return Some($name::$id) } )*
                
                None
            }

            fn from_value(value: &$valtype) -> Option<Self> {
                $( if $name::$id.equal(value) { return Some($name::$id) } )*
                
                None
            }
        }
    };
}
