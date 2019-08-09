
#[macro_export]
macro_rules! s {
    ($t:expr) => {
        String::from($t)
    };
}

#[macro_export]
macro_rules! dt {
    ($t:expr) => {
        chrono::DateTime::parse_from_rfc3339($t)
    };
}

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
