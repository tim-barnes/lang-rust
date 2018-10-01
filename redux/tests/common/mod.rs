enum Messages {
    Add(i32)
}

struct SomeState(i32);

impl State for SomeState {
    type StateType = SomeState;

    fn to_string(&self) -> String {
        format!("State = {}", self.0)
    }
}


struct SomeReducer;
impl Reducer for SomeReducer {
    type StateType = SomeState;
    type MessageType = Messages;

    fn reduce(&self, state: &Self::StateType, message: &Self::MessageType) -> Self::StateType {
        match message {
            Messages::Add(x) => Self::StateType(state.0 + x),
        }
    }
}


fn create_store() -> Store<SomeState, Messages> {
    let mut store = Store::new(SomeState(0));
    store.push_reducer(SomeReducer);
    return store;
}