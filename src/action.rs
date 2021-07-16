mod rllib {
    pub struct Action<T> {
        action_id: i32,
        values: Vec<T>,
    }
    impl<T> Copy for Action<T> where T: Copy{
       
    }
    impl<T> Clone for Action<T> {
        fn clone(&self) -> Action<T> {
            *self
        }
    }
    impl<T> Action<T> {
        
        pub fn push_back(&mut self, value: T) {
            self.values.push(value);
        }
    
        pub fn get_entry(&mut self, i: usize) -> T {
            
            return self.values[i];
        }
    }
    
}
