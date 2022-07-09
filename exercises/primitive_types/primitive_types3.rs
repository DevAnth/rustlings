// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

fn make_element<T>(elem: T) -> T
{
    return elem;
}

fn main() {
    //let a = [0u32..100];
    let a = [make_element(0u32)..make_element(100)];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
