// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` for hints :)



fn main() {
    let mut vec0 = Vec::new();

    // 1. let mut vec1 = fill_vec(vec0.clone());
    // 2. let mut vec1 = fill_vec(&vec0);
    // 3.
    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // 3.
    vec0.push(88);

    // 3.
    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

// 3.
fn fill_vec(vec: &mut Vec<i32>) {
    // 3. let mut vec = vec;
    // 2. let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    // 3. vec
}
