#[macro_export]
macro_rules! vec2 {
    [$
        ($t:expr)
    ,+ $(,)*] => {
        {
            let mut v = Vec::new();
            $(
                v.push($t.to_vec());
            )*
            v
        }
    };
}
