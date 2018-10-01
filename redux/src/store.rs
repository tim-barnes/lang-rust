struct Store<S: State, M>(
    Box<S>, 
    Vec<Box<Reducer<StateType=S, MessageType=M>>>);

impl <S: State, M> State for Store<S, M> {
    type StateType = S;

    fn to_string(&self) -> String {
        return (&*self.0).to_string();
    }
}

impl <S: State, M> Store<S, M>  {
    fn reduce(&mut self, message: &M) {
        for i in &mut self.1 {
            self.0 = Box::new(i.reduce(&*self.0, message));
        }
    }

    fn push_reducer(&mut self, reducer: Reducer<StateType=S, MessageType=M>) {
        self.1.push(reducer);
    }
}
