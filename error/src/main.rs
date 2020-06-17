// Experiment with structs and macros!

struct Error(String);

macro_rules! error_msg {
    ($s: expr, $( $y: expr ),* ) => {{
        Error(format!($s, $($y,)*))
    }}
}




fn main() {
    println!("Hello, world!");

    let fish = "cod";
    let _i = error_msg!("Hello, i'm an {}, i'm {}, and I like {}", "ERROR", 12, fish);

    //  SPLIT: 13.032 = 4.785 + 8.216 + 0.031
    let c: f32 = 13.032;
    let c1: f32 = 4.785;
    let c2: f32 = 8.216;
    let w: f32 = 0.031;
    let s: f32 = c1 + c2 + w;

    println!("c: {}", c);
    println!("c1: {} c2: {} w: {}", c1, c2, w);
    println!("Sum: {} {}", s, s <= c);


    let c: f64 = 13.032;
    let c1: f64 = 4.785;
    let c2: f64 = 8.216;
    let w: f64 = 0.031;
    let s: f64 = c1 + c2 + w;

    println!("c: {}", c);
    println!("c1: {} c2: {} w: {}", c1, c2, w);
    println!("Sum: {} {}", s, s <= c);



    let v: Vec<i32> = vec![1, 2, -3, 4];


    let b: Vec<bool> = v.clone().into_iter().map(|x| {
        if x < 0 {
            Err(())
        }
        x >= 0
    }).collect();

    println!("B: {:?}", b);
}
