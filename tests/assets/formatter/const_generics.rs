fn foo<const N: usize>() {}

fn bar<T, const M: usize>() {
    foo::<M>();
    foo::<2021>();
    foo::<{ 20 * 100 + 20 * 10 + 1 }>();

    foo::<{ M + 1 }>();
    foo::<{ std::mem::size_of::<T>() }>();

    let _: [u8; M];
    let _: [u8; std::mem::size_of::<T>()];
}

fn main() {}
