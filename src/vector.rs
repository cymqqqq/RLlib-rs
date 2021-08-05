mod rllib {
  pub enum VectorType {
        BASE_VECTOR,
        DENSE_VECTOR,
        SPARSE_VECTOR,
    }
    pub struct Vector {
        vectorType: VectorType,
    }
    pub trait VectorTrait<T> {
        fn dimmension() -> i32;
        fn empty() -> bool;
        fn max_norm() -> T;
        fn l1_norm() -> T;
        fn l2_norm() -> T;
        fn sum() -> T;
        fn get_value() -> Box<T>;
        
    }
}
