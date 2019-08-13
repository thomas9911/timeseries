/// macro that creates a Vec<Vec<V>> from an array of arrays with elements V
/// ```
/// use timeseries::vec2;
/// let v = vec2![
///     ["1", "Test01", "test", "abcd"],
///     ["2", "Test02", "test", "efgh"],
///     ["3", "Test03", "test", "ijkl"],
///     ["4", "Test04", "test", "mnop"],
///     ["5", "Test05", "test", "qrst"],
///     ["6", "Test06", "test", "uvwx"],
/// ];
///
/// let d = [
///     ["1", "Test01", "test", "abcd"].to_vec(),
///     ["2", "Test02", "test", "efgh"].to_vec(),
///     ["3", "Test03", "test", "ijkl"].to_vec(),
///     ["4", "Test04", "test", "mnop"].to_vec(),
///     ["5", "Test05", "test", "qrst"].to_vec(),
///     ["6", "Test06", "test", "uvwx"].to_vec(),
/// ].to_vec();
/// assert_eq!(v, d);
/// ```
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

///
/// ```
/// use timeseries::map;
/// use std::collections::HashMap;
///
/// let mut expected = HashMap::new();
/// expected.insert("test", "123");
/// expected.insert("test2", "456");
///
/// let t = map!{
///     "test" => "123",
///     "test2" => "456"
/// };
///
/// assert_eq!(expected, t);
/// ```
#[macro_export]
macro_rules! map {
    {$
        (
            $t:expr => $s:expr
        ),+ $(,)*
    } => {
        {
            use std::collections::HashMap;

            let mut v = HashMap::new();
            $(
                v.insert($t, $s);
            )*
            v
        }
    };
}
