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
    impl std::cmp::PartialEq<Action<i32>> for Action<i32> {
        fn eq(&self, other: &Self) -> bool {
            self.action_id == other.action_id
        }
        fn ne(&self, other: &Self) -> bool {
            self.action_id != other.action_id
        }
    } 
    impl ActionTrait<i32> for Action<i32> {
        fn push_back(&mut self, value: i32) {
            self.values.push(value);
        }
        fn get_entry(&mut self, i: usize) -> i32 {
            return self.values[i];
        }
        fn dim(&self) -> i32 {
            return self.values.len() as i32;
        }
        fn update(&mut self, i: usize, value: i32) {
            self.values[i] = value;
        }
        fn id(&self) -> i32 {
            return self.action_id;
        }

    }
    impl ActionTrait<f32> for Action<f32> {
        fn push_back(&mut self, value: f32) {
            self.values.push(value);
        }
        fn get_entry(&mut self, i: usize) -> f32 {
            return self.values[i];
        }
        fn dim(&self) -> i32 {
            return self.values.len() as i32;
        }
        fn update(&mut self, i: usize, value: f32) {
            self.values[i] = value;
        }
        fn id(&self) -> i32 {
            return self.action_id;
        }

    }
    impl ActionTrait<f64> for Action<f64> {
        fn push_back(&mut self, value: f64) {
            self.values.push(value);
        }
        fn get_entry(&mut self, i: usize) -> f64 {
            return self.values[i];
        }
        fn dim(&self) -> i32 {
            return self.values.len() as i32;
        }
        fn update(&mut self, i: usize, value: f64) {
            self.values[i] = value;
        }
        fn id(&self) -> i32 {
            return self.action_id;
        }

    }
    impl ActionTrait<String> for Action<String> {
        fn push_back(&mut self, value: String) {
            self.values.push(value);
        }
        fn get_entry(&mut self, i: usize) -> String {
            return self.values[i].clone();
        }
        fn dim(&self) -> i32 {
            return self.values.len() as i32;
        }
        fn update(&mut self, i: usize, value: String) {
            self.values[i] = value;
        }
        fn id(&self) -> i32 {
            return self.action_id;
        }

    }
    impl ActionTrait<char> for Action<char> {
        fn push_back(&mut self, value: char) {
            self.values.push(value);
        }
        fn get_entry(&mut self, i: usize) -> char {
            return self.values[i];
        }
        fn dim(&self) -> i32 {
            return self.values.len() as i32;
        }
        fn update(&mut self, i: usize, value: char) {
            self.values[i] = value;
        }
        fn id(&self) -> i32 {
            return self.action_id;
        }

    }


    
}
