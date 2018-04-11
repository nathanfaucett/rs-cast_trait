use core::num::Wrapping;

pub trait Cast<T> {
    /// # Example
    /// ```
    /// use cast_trait::Cast;
    /// use std::num::Wrapping;
    ///
    /// assert_eq!(Cast::<isize>::cast(2_usize) * 2_isize, 4_isize);
    /// assert_eq!(Cast::<f32>::cast(2_u32) * 2_f32, 4_f32);
    /// assert_eq!(Cast::<bool>::cast(1_u32), true);
    /// assert_eq!(Cast::<f32>::cast(false), 0_f32);
    /// assert_eq!(Cast::<usize>::cast(true), 1_usize);
    /// assert_eq!(
    ///     Cast::<Wrapping<usize>>::cast(Wrapping(1.0)),
    ///     Wrapping(1_usize)
    /// );
    /// ```
    fn cast(self) -> T;
}

macro_rules! impl_cast {
    ($F:ty, $($T:ty),*) => ($(
        impl Cast<$T> for $F {
            #[inline(always)]
            fn cast(self) -> $T {
                self as $T
            }
        }
    )*);
}

macro_rules! impl_cast_primitive {
    ($($F:ty),*) => (
        $(
            impl_cast!(
                $F,
                i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64
            );
            #[cfg(feature = "nightly")]
            impl_cast!(
                $F,
                i128, u128
            );
        )*
    );
}

impl_cast_primitive!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

#[cfg(feature = "nightly")]
impl_cast_primitive!(i128, u128);

macro_rules! impl_cast_bool {
    (int, $($T:ty),*) => (
        $(
            impl Cast<$T> for bool {
                #[inline(always)]
                fn cast(self) -> $T {
                    if self {1} else {0}
                }
            }
            impl Cast<bool> for $T {
                #[inline(always)]
                fn cast(self) -> bool {
                    if self == 0 {false} else {true}
                }
            }
        )*
    );
    (float, $($T:ty),*) => (
        $(
            impl Cast<$T> for bool {
                #[inline(always)]
                fn cast(self) -> $T {
                    if self {1.0} else {0.0}
                }
            }
            impl Cast<bool> for $T {
                #[inline(always)]
                fn cast(self) -> bool {
                    if self == 0.0 {false} else {true}
                }
            }
        )*
    );
}

impl_cast_bool!(int, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
impl_cast_bool!(float, f32, f64);

#[cfg(feature = "nightly")]
impl_cast_bool!(int, i128, u128);

impl<A, B> Cast<Wrapping<B>> for Wrapping<A>
where
    A: Cast<B>,
{
    #[inline(always)]
    fn cast(self) -> Wrapping<B> {
        Wrapping(Cast::cast(self.0))
    }
}

macro_rules! reverse {
    ($self:ident [] $($reversed:expr),+) => (
        [$($reversed),+]
    );
    ($self:ident []) => ([]);
    ($self:ident [$first:expr]) => (
        reverse!($self [] $crate::Cast::cast($self[$first].clone()))
    );
    ($self:ident [$first:expr, $($rest:expr),+]) => (
        reverse!($self [$($rest),+] $crate::Cast::cast($self[$first].clone()))
    );
    ($self:ident [$first:expr] $($reversed:expr),+) => (
        reverse!($self [] $crate::Cast::cast($self[$first].clone()), $($reversed),+)
    );
    ($self:ident [$first:expr, $($rest:expr),+] $($reversed:expr),+) => (
        reverse!($self [$($rest),+] $crate::Cast::cast($self[$first].clone()), $($reversed),+)
    );
}

macro_rules! count_args {
    () => { 0 };
    ($x:expr) => { 1 };
    ($x:expr, $($y:expr),+) => { 1 + count_args!($($y),+) };
}

macro_rules! impl_cast_slice {
    ($($x:expr),+) => (
        impl<A, B> Cast<[B; count_args!($($x),+)]> for [A; count_args!($($x),+)]
        where
            A: Clone + Cast<B>,
        {
            #[inline]
            fn cast(self) -> [B; count_args!($($x),+)] {
                reverse!(self [$($x),+])
            }
        }
    );
}

macro_rules! impl_cast_slices {
    () => ();
    ($x:expr) => (
        impl_cast_slice!($x);
    );
    ($x:expr, $($y:expr),+) => (
        impl_cast_slice!($x, $($y),+);
        impl_cast_slices!($($y),+);
    );
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl_cast_slices!(
    31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0
);

#[test]
fn test_cast_slice() {
    let mut x: [i32; 32] = [0_i32; 32];

    for i in 0..32 {
        x[i] = i as i32;
    }

    let mut y: [f32; 32] = [0_f32; 32];

    for i in 0..32 {
        y[i] = i as f32;
    }

    let z: [f32; 32] = x.cast();
    assert_eq!(z, y);
}

#[cfg(feature = "nightly")]
#[test]
fn test_i128_u128() {
    let x: i128 = 1;
    let y: u128 = x.cast();
    let z: f32 = y.cast();
    assert_eq!(x, 1_i128);
    assert_eq!(y, 1_u128);
    assert_eq!(z, 1_f32);
}
