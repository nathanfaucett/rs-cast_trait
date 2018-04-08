# rs-cast_trait

=====

cast trait

```rust
use cast_trait::Cast;

fn cast<A, B>(a: A) -> B
where A: Cast<B>,
{
	a.cast()
}

assert_eq!(cast::<i32, f32>(10_i32), 10_f32);
```
