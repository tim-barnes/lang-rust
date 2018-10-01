pub trait State {
    type StateType;

    fn to_string(&self) -> String;
}
