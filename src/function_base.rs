mod rllib {
    pub trait ParaFunc {
        fn persist(&self, f: String);
        fn resurrect(&self, f: String);
    }
    pub trait Learner<T>
    where T: Clone + ParaFunc
    {
        fn init(&self) -> T;
        fn reset(&self);
    }
    pub trait RewardFunc<T> {
        fn reward(&self) -> T;
    }
    pub trait OutcomeFunc<T> {
        fn outcome(&self) -> T;
    }
}
