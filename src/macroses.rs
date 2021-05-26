#[macro_export]
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

#[macro_export]
macro_rules! struct_w_weight {
    ($name:ident, weight=$weight:tt, count=$count:tt) => {
        struct $name {
            weight: i16,
            count:  u16
        }
        impl $name {
            fn new() -> Self{
                Self { weight: $weight, count: $count }
            }
        }
        impl std::ops::AddAssign<u16> for $name {
            fn add_assign(&mut self, count: u16) {
                self.count += count;
            }
        }
    }
}
