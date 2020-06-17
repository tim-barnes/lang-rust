

enum State<T> {
    Leaf(T),
    // Node(HashMap<String, State<T>>),
    // None
}


pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for State<String> {
    fn to_string(&self) -> String {
        match self {
            State::Leaf(s) => s.clone(),
        }
    }
}

impl ToString for State<i32> {
    fn to_string(&self) -> String {
        match self {
            State::Leaf(v) => format!("{}", v),
        }
    }
}


fn test(i: u32) -> Result<bool, String> {
    if i % 2 == 0 {
        Result::Ok(true)
    } else {
        Result::Err(String::from("BAD VALUE"))
    }
}


fn main() {
    let s = State::Leaf(String::from("123"));
    let t = State::Leaf(123);
    // println!("{}", s);
    println!("String:  {}", s.to_string());
    println!("u32:     {}", t.to_string());


    if let Ok(x) = test(10) {
        println!("x is {}", x);
    }
}
