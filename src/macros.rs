#[macro_export]
macro_rules! chord {
    ( $( $i:expr ),* ) => {
        {
            let mut temp_scale = $crate::Chord::new();
            $(
                temp_scale.insert($i);
            )*
                temp_scale
        }
    };
}

/// ***Panics*** if a provided interval is out of bounds.
#[macro_export]
macro_rules! scale {
    ( $( $i:expr ),* ) => {
        {
            let mut temp_scale = $crate::Scale::new();
            $(
                temp_scale.insert($i);
            )*
                temp_scale
        }
    };
}
