mod rllib {
    pub struct Action<T>
    {
        action_id: i32,
        values: Vec<T>,
    }

    pub trait ActionTrait<T> {
        fn push_back(&mut self, value: T);
        fn get_entry(&mut self, i: usize) -> T;
        fn dim(&self) -> i32;
        fn update(&mut self, i: usize, value: T);
        fn id(&self) -> i32;
    }
    impl<T> ActionTrait<T> for Action<T> {
        fn push_back(&mut self, value: T) {
            self.values.push(value);
        }
        fn get_entry(&mut self, i: usize) -> T {
            return self.values[i];
        }
        fn dim(&self) -> i32 {
            return self.values.len() as i32;
        }
        fn update(&mut self, i: usize, value: T) {
            self.values[i] = value;
        }
        fn id(&self) -> i32 {
            return self.action_id;
        }

    }
    impl<T> std::cmp::PartialEq<Action<T>> for Action<T> {
        fn eq(&self, other: &Self) -> bool {
            self.action_id == other.action_id
        }
        fn ne(&self, other: &Self) -> bool {
            self.action_id != other.action_id
        }
    } 
    

    pub trait ActionsTrait<T> 
    {
        fn dimension(&self) -> i32;
        fn get_entry(&self, idx: usize) -> &Box<Action<T>>;
        fn push_back(&mut self, idx: usize, val: T);
        //fn erase(&self, idx: i32);
        fn update(&mut self, actionidx: usize, vectoridx: usize, val: T);
    }
    impl<T> ActionsTrait<T> for ActionArr<T> {
        fn push_back(&mut self, idx: usize, val: T) {
            self.Base.actions[idx].push_back(val);
        }
        fn get_entry(&self, idx: usize) -> &Box<Action<T>> {
            return &self.Base.actions[idx];
        }
        fn dimension(&self) -> i32 {
            return self.Base.actions.len() as i32;
        }
        fn update(&mut self, actionidx: usize, vectoridx: usize, val: T) {
            self.Base.actions[actionidx].update(vectoridx, val);
        }
    }
    pub struct Actions<T> {
        actions: Vec<Box<Action<T>>>,
    }
    pub struct ActionArr<T> {
        base: Actions<T>,
    }
    pub trait ActionsTrait<T>
    where T: Clone,
    {
        fn dimension(&self) -> i32;
        fn get_entry(&self,idx: usize) -> Box<Action<T>>;
        fn push_back(&mut self, idx: usize, val: T);
        fn erase(&self, idx: i32);
        fn update(&mut self, actionidx: usize, vectoridx: usize, val: T);
    }
    impl ActionsTrait<T> for ActionArr<T> {
        fn push_back(&mut self, idx: usize, val: T) {
            self.base.actions[idx].push_back(val);
        }
        fn get_entry(&self, idx: usize) -> &Box<Action<T>> {
            return &self.base.actions[idx];
        }
        fn dimension(&self) -> i32 {
            return self.base.actions.len() as i32;
        }
        fn update(&mut self, actionidx: usize, vectoridx: usize, val: T) {
            self.base.actions[actionidx].update(vectoridx, val);
        }
    }
    
}
