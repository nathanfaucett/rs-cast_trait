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

macro_rules! trait_cast {
    ($F:ty, $($T:ty),*) => (
        $(impl Cast<$T> for $F {
            #[inline(always)]
            fn cast(self) -> $T {
                self as $T
            }
        })*
    );
}

macro_rules! trait_primitive_cast {
    ($($F:ty),*) => (
        $(
            trait_cast!(
                $F,
                i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
            );
        )*
    );
}

trait_primitive_cast!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);
trait_primitive_cast!(i128, u128);

macro_rules! trait_cast_bool {
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

trait_cast_bool!(int, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
trait_cast_bool!(int, i128, u128);
trait_cast_bool!(float, f32, f64);

impl<A, B> Cast<Wrapping<B>> for Wrapping<A>
where
    A: Cast<B>,
{
    #[inline(always)]
    fn cast(self) -> Wrapping<B> {
        Wrapping(Cast::cast(self.0))
    }
}
