// traits2.rs
//
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!

trait AppendBarRef {
    fn append_bar_ref(&mut self) -> &Self;
}

trait AppendBar {
    fn append_bar(self) -> Self;
}

//TODO: Add your code here

impl AppendBar for Vec<String>
{
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

impl AppendBarRef for Vec<String>
{
    fn append_bar_ref(&mut self) -> &Self {
        self.push(String::from("Bar"));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }

    #[test]
    fn testing_by_ref() {
        let mut foo = vec![String::from("Foo")];
        foo.append_bar_ref();

        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
