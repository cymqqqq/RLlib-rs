mod rllib {
    pub struct Vector<T>(T);
    
    pub enum VectorType {
        BASE_VECTOR,
        DENSE_VECTOR,
        SPARSE_VECTOR,
    }
    /*
    pub struct Vector {
        vectortype: VectorType,
    }
    */
    pub trait VectorTrait<T> {
        fn dimension(&self) -> i32;
        fn empty(&self) -> bool;
        fn max_norm(&self) -> T;
        fn l1_norm(&self) -> T;
        fn l2_norm(&self) -> T;
        fn sum(&self) -> T;
        fn get_values(&self) -> Box<T>;
        fn get_entry(&self, idx: usize) -> T;
        fn dot(&mut self, other: Box<Vector<T>>) -> T;
        fn clear(&mut self);
        fn set_entry(&mut self, idx: i32, val: T);
        fn remove_entry(&mut self, idx: i32);
        fn addToself(&mut self, val: T) -> Box<Vector<T>>;
        //fn addToself(&mut self, factor: T, other: Box<Vector<T>>) -> Box<Vector<T>>;
        //fn addToself(&mut self, other: Box<Vector<T>>) -> Box<Vector<T>>;
        fn substractToself(&mut self, other: Box<Vector<T>>) -> Box<Vector<T>>;
        fn mapMultiplyToself(&mut self, factor: T) -> Box<Vector<T>>;
        fn set(&mut self, other: Box<Vector<T>>) -> Box<Vector<T>>;
        //fn set(&mut self, other: Box<Vector<T>>, offset: i32) -> Box<Vector<T>>;
        //fn set(&mut self, val: T) -> Box<Vector<T>>;
        //deep copy of vector
        fn copy(&mut self) -> Box<Vector<T>>;
        fn newInstance(&mut self, dim: i32) -> Box<Vector<T>>;
        //storage management
        fn persist(&mut self, f: String);
        fn resurrect(&mut self, f: String);
    }
    pub struct DenseVector<T> {
        capacity: i32,
        data: Box<T>,
    }
}
