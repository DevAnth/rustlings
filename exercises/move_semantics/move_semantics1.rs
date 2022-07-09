// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)


fn main() {
    let mut vec0 = Vec::new();
    fill_vec(vec0.as_mut());

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>)
{
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
