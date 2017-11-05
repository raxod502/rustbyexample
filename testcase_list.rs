use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (idx, v) in vec.iter().enumerate() {
            if idx != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}: {}", idx, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![42, 81, 70]);
    println!("{}", v);
}
