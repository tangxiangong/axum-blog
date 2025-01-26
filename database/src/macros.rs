#[macro_export]
macro_rules! set_value {
    ($admin:ident, $(($field:ident, $value:expr $(, $mode:ident)?)),* $(,)?) => {
        $(
            if let Some(ref val) = $value {
                $admin.$field = Set($crate::set_value!(@inner val $(, $mode)?));
            }
        )*
    };
    (@inner $val:expr) => {
        Some($val.clone())
    };
    (@inner $val:expr, direct) => {
        $val.clone()
    };
}
