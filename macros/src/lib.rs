#[macro_export]
macro_rules! hashmap {
    // macro for no arguments
    () => {
        {
            let mut hm = ::std::collections::HashMap::new();
            hm
        }
    };
    // macro for one or more arguments
    ( $( $x:expr => $y:expr ),+ $(,)? ) => {
        {
            let mut hm = ::std::collections::HashMap::new();
            $(
                hm.insert($x, $y);
            )*
            hm
        }
    };
}
