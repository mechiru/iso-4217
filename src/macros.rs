#[doc(hidden)]
#[macro_export]
macro_rules! impl_curr_code {
    ($(#[$attr:meta])* ($($vis:tt)*)
     enum $name:ident {
         $($var:ident { num: $num:expr, digit: $digit:expr, name: $currency:expr, },)*
     }) => {
        $(#[$attr])*
        $($vis)* enum $name { $($var = $num),* }

        impl $name {
            /// `alphabetic code` represented by three letters.
            pub fn alpha(&self) -> &'static str {
                match self { $($name::$var => stringify!($var),)* }
            }

            /// `numeric code` represented by a three-digit number code.
            pub fn num(&self) -> u32 {
                match self { $($name::$var => $num,)* }
            }

            /// Currency name.
            pub fn name(&self) -> &'static str {
                match self { $($name::$var => $currency,)* }
            }

            /// The number of digits after the decimal separator.
            pub fn digit(&self) -> Option<u32> {
                match self { $($name::$var => $digit,)* }
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = crate::error::ParseCodeError;
            fn from_str(v: &str) -> Result<Self, Self::Err> {
                ::std::convert::TryFrom::try_from(v)
            }
        }

        impl ::std::convert::TryFrom<&str> for $name {
            type Error = crate::error::ParseCodeError;
            fn try_from(v: &str) -> Result<Self, Self::Error> {
                match v {
                    $(stringify!($var) => Ok($name::$var),)*
                    v => Err(crate::error::ParseCodeError::Alpha(v.into())),
                }
            }
        }

        impl ::std::convert::TryFrom<u32> for $name {
            type Error = crate::error::ParseCodeError;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    $($num => Ok($name::$var),)*
                    v => Err(crate::error::ParseCodeError::Num(v)),
                }
            }
        }

        impl ::std::convert::From<$name> for &str {
            fn from(v: $name) -> Self {
                v.alpha()
            }
        }

        impl ::std::convert::From<$name> for u32 {
            fn from(v: $name) -> Self {
                v.num()
            }
        }
    }
}
