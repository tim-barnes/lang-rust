//use std::ops::Deref;

// Some state
struct SomeState {
    i: i32,
    s: String,
    d: f64
}

trait State {
    type StateType;

    fn to_string(&self) -> String;
}


impl State for SomeState {
    type StateType = SomeState;

    fn to_string(&self) -> String {
        format!("State:\n    i: {}\n    s: {}\n    d: {}",
                self.i,
                self.s,
                self.d)
    }
}


enum Messages {
    Add(i32),
    Multiply(i32),
}



trait Reducer {
    type StateType;
    type MessageType;

    fn reduce(&self, state: &Self::StateType, message: &Self::MessageType) -> Self::StateType;
}


struct SomeReducer;
impl Reducer for SomeReducer {
    type StateType = SomeState;
    type MessageType = Messages;

    fn reduce(&self, state: &Self::StateType, message: &Self::MessageType) -> Self::StateType {
        match message {
            Messages::Add(x) => Self::StateType {
                i: state.i + x,
                s: state.s.clone(),
                d: state.d + (*x as f64),
            },
            Messages::Multiply(x) => Self::StateType {
                i: state.i * x,
                s: state.s.clone(),
                d: state.d * (*x as f64),
            }
        }
    }
}


struct StateWrapper<S: State>(Box<S>, Vec<Box<Reducer<StateType=S, MessageType=Messages>>>);

impl <S: State> State for StateWrapper<S> {
    type StateType = S;

    fn to_string(&self) -> String {
        return (&*self.0).to_string();
    }

}

impl <S: State> StateWrapper<S>  {
    fn reduce(&mut self, message: &Messages) {
        for i in &mut self.1 {
            self.0 = Box::new(i.reduce(&*self.0, message));
        }
    }
}


fn main() {

    let mut s = SomeState {
        i: 123,
        s: String::from("Hello World!"),
        d: 1.23456
    };

    let mut b = Box::new(SomeState {
        i: 890,
        s: String::from("Goodbye cruel world!"),
        d: 2.345E-1,
    });

    println!("{}", s.to_string());
    println!("{}", (*b).to_string());

    // See if a naive reducer works

    println!("--- Reducer PlusOne ---");

    let r = SomeReducer;
    let msg = Messages::Add(123);

    s = r.reduce(&s, &msg);
    b = Box::new(r.reduce(&*b, &msg));

    println!("{}", s.to_string());
    println!("{}", (*b).to_string());


    println!("--- StateWrapper ---");
    let mut reducers: Vec<Box<Reducer<StateType=SomeState, MessageType=Messages>>> = Vec::new();
    reducers.push(Box::new(SomeReducer));

    let mut w = StateWrapper(Box::new(s), reducers);
    println!("*** Before: \n{}", w.to_string());
    w.reduce(&Messages::Multiply(10));
    w.reduce(&Messages::Add(1));
    println!("*** After: \n{}", w.to_string());
}
