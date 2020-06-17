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
    fn as_state(&self) -> &Self::StateType;
}


impl State for SomeState {
    type StateType = SomeState;

    fn to_string(&self) -> String {
        format!("State:\n    i: {}\n    s: {}\n    d: {}",
                self.i,
                self.s,
                self.d)
    }

    fn as_state(&self) -> &Self::StateType {
        return self;
    }
}

fn state_str<T: State>(s: &T) -> String
{
    return s.to_string();
}


trait Reducer {
    type StateType;

    fn reduce(&self, state: &Self::StateType) -> Self::StateType;
}


struct PlusOne;
impl Reducer for PlusOne {
    type StateType = SomeState;

    fn reduce(&self, state: &Self::StateType) -> Self::StateType {
        return Self::StateType {
            i: state.i + 1,
            s: state.s.clone(),
            d: state.d + 1.0,
        }
    }
}

struct TimesTen;
impl Reducer for TimesTen {
    type StateType = SomeState;

    fn reduce(&self, state: &Self::StateType) -> Self::StateType {
        return Self::StateType {
            i: state.i * 10,
            s: state.s.clone(),
            d: state.d * 10.0,
        }
    }
}


struct StateWrapper<S: State>(Box<S>, Vec<Box<Reducer<StateType=S>>>);

impl <S: State> State for StateWrapper<S> {
    type StateType = S;

    fn to_string(&self) -> String {
        return (&*self.0).to_string();
    }

    fn as_state(&self) -> &Self::StateType {
        return &*self.0;
    }
}

impl <S: State> StateWrapper<S>  {
    fn reduce(&mut self) {
        for i in &mut self.1 {
            self.0 = Box::new(i.reduce(&*self.0));
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

    println!("{}", state_str(&s));
    println!("{}", state_str(&*b));

    // See if a naive reducer works

    println!("--- Reducer PlusOne ---");

    let r = PlusOne;

    s = r.reduce(&s);
    b = Box::new(r.reduce(&*b));

    println!("{}", state_str(&s));
    println!("{}", state_str(&*b));


    println!("--- StateWrapper ---");
    let mut reducers: Vec<Box<Reducer<StateType=SomeState>>> = Vec::new();
    reducers.push(Box::new(TimesTen));
    reducers.push(Box::new(PlusOne));

    let mut w = StateWrapper(Box::new(s), reducers);
    println!("*** Before: \n{}", w.to_string());
    w.reduce();
    println!("*** After: \n{}", w.to_string());
}
