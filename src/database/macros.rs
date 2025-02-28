#[macro_export]
macro_rules! set_value {
    ($admin:ident, $(($field:ident, $value:expr_2021 $(, $mode:ident)?)),* $(,)?) => {
        $(
            if let Some(ref val) = $value {
                $admin.$field = Set($crate::set_value!(@inner val $(, $mode)?));
            }
        )*
    };
    (@inner $val:expr_2021) => {
        Some($val.clone())
    };
    (@inner $val:expr_2021, direct) => {
        $val.clone()
    };
}
