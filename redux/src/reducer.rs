
pub trait Reducer {
    type StateType;
    type MessageType;

    fn reduce(&self, state: &Self::StateType, message: &Self::MessageType) -> Self::StateType;
}
