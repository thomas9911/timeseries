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
