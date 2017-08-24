#[macro_export]
macro_rules! last_element {
    ($v: ident) => (
        $v[$v.len() - 1]
    )
}