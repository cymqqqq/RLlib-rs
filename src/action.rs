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
    impl ActionTrait<u8> for Action<u8> {
        fn push_back(&mut self, value: u8) {
            self.values.push(value);
        }
        fn get_entry(&mut self, i: usize) -> u8 {
            return self.values[i];
        }
        fn dim(&self) -> i32 {
            return self.values.len() as i32;
        }
        fn update(&mut self, i: usize, value: u8) {
            self.values[i] = value;
        }
        fn id(&self) -> i32 {
            return self.action_id;
        }

    }
    


    
}
