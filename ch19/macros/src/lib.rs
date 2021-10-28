#[macro_export]
macro_rules! sum {
    ( $( $num:expr ),* ) => {
        {
            let mut total = 0;
            $(
                total += $num;
            )*
            total
        }
    };
}