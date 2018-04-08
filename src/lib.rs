#![no_std]

mod cast;

pub use self::cast::Cast;

#[inline(always)]
pub fn cast<A, B>(a: A) -> B
where
    A: Cast<B>,
{
    a.cast()
}
