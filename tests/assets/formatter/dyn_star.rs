fn _foo() -> dyn* Unpin {
    4usize
}

pub fn dyn_star_parameter(_: dyn* Send) {

}
