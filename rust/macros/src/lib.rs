#[macro_export]
macro_rules! hashmap {
    ( $( $i:expr => $j:expr ),* $(,)? ) => {
        {
            let mut hm = std::collections::HashMap::new();
            $(
                hm.insert($i, $j);
            )*
            hm
        }
    };
}
