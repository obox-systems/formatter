pub fn main() {
    let f: ([isize; 3],) = ([5, 6, 7],);
    let g: &([isize],) = &f;
    let h: &(([isize],),) = &(*g,);
}
