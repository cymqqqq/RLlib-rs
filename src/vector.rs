use std::ops::Index;
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
        //fn l2_norm(&self) -> T;
        fn sum(&mut self) -> T;
        fn get_values(&self) -> Vec<T>;
        fn get_entry(&self, idx: usize) -> T;
        //fn dot(&mut self, other: Vector<T>) -> T;
        fn get(&self, idx: usize) -> T;
        fn clear(&mut self);
        fn set_entry(&mut self, idx: usize, val: T);
        fn remove_entry(&mut self, idx: usize);
        fn addToself(&mut self, val: T) -> Vec<T>;
        //fn substractToself(&mut self, other: Vec<T>) -> Vec<T>;
        fn mapMultiplyToself(&mut self, factor: T) -> Vec<T>;
        //fn set(&mut self, other: Vec<T>) -> Vec<T>;
        //fn set(&mut self, other: Vec<T>, offset: i32) -> Vec<T>;
        fn set(&mut self, val: T) -> Vec<T>;
        //deep copy of vector
        //fn copy(&mut self) -> Box<Vector<T>>;
        //fn newInstance(&mut self, dim: i32) -> Box<Vector<T>>;
        //storage management
        //fn persist(&mut self, f: String);
        //fn resurrect(&mut self, f: String);
    }
    pub trait VectorTraitaddToself1<T> {
        fn addToself(&mut self, factor: T, other: Box<Vector<T>>) -> Box<Vector<T>>;
    }
    pub trait VectorTraitaddToself2<T> {
        fn addToself(&mut self, other: Box<Vector<T>>) -> Box<Vector<T>>;
    }
    pub struct DenseVector<T> {
        vecdata: Vector<T>,
        capacity: i32,
        data: Vec<T>,
    }
    impl<T> Index<usize> for DenseVector<T> {
        type Output = T;
        fn index(&self, idx: usize) -> &T {
            &self.get(idx)
        }
    }
    
    impl VectorTrait<f32> for DenseVector<f32> {
        fn dimension(&self) -> i32 {
            return self.capacity;
        }
        fn empty(&self) -> bool {
            return (dimension() == 0i32);
        }
        fn max_norm(&self) -> f32 {
            let mut maxv: f32 = if self.capacity > 0i32 { self.data[0].abs() } else { 0f32 }
            if self.capacity > 0 {
                for i in 0..self.capacity {
                    if self.data[i].abs() > maxv {
                        maxv = self.data[i].abs();
                    }
                }
            }
            return maxv;
        }
        fn l1_norm(&self) -> f32 {
            let mut result = 0f32;
            for i in 0..self.capacity {
                result += self.data[i].abs();
            }
            return result;
        }
        fn sum(&mut self) -> f32 {
            return self.data.sum();
        }
        fn get_values(&self) -> Vec<f32> {
            return self.data;
        }
        fn get_entry(&self, idx: usize) -> f32 {
            return self.data[idx];
        }
        fn clear(&mut self) {
            self.data.clear();
        }
        fn set_entry(&mut self, idx: i32, val: f32) {
            self.data[idx as usize] = val;
        }
    }
    pub struct SparseVector<T> {
        idxPosLen: usize,
        activeIdxLen: usize,
        nbActive: usize,
        idxPos: Vec<Box<usize>>,
        activeIdx: Vec<Box<usize>>,
        val: Vec<Box<T>>,
    }
    impl SparseVector<T> {
        fn new(capacity: usize = 1, actidx: usize = 10, 
        activeIdxLen: usize, ) -> Self {
            Self {
                idxPosLen: capacity,
                activeIdxLen: activeIdxLen,
                nbActive: 0i32,
                idxPos: vec![],
                activeIdx: vec![],
                val: vec![],
            }
        }
    }
    pub trait SparseVectorTrait<T> {
        fn update_entry(&mut self, idx: usize, val: Box<T>, pos: usize);
        fn swap_entry(&mut self, pos1: usize, pos2: usize);
        fn remove_entry(&mut self, pos: usize);
        fn remove(&mut self, idx: usize);
        fn set_entry(&mut self, idx: usize, val: Box<T>);
        fn append_entry(&mut self, idx: usize, val: Box<T>);
        fn insert_entry(&mut self, idx: usize, val: Box<T>);
        fn get_entry(&self, idx: usize) -> Box<T>;
        fn setnonzero_entry(&mut self, idx: usize, val: Box<T>);
        fn clear(&mut self);
        fn sum(&mut self) -> Vec<Box<T>>;
        fn nonzeroidx(&mut self) -> Vec<Box<usize>>;
        fn getidxpos(&mut self) -> Vec<Box<usize>>;
        fn nonzeroelem(&mut self) -> usize;
        fn dimension(&self) -> usize;
        fn empty(&self) -> bool;
        fn max_norm(&mut self) -> Box<T>;
        fn l1_norm(&mut self) -> Box<T>;
        fn get_val(&self) -> Vec<Box<T>>;
        fn dot_prod(&mut self, data: Vec<Box<T>>) -> Box<T>;
        fn addSelfTo(&mut self, factor: Box<T>, data: Vec<Box<T>>);
        fn subSelfTo(&mut self, data: Vec<Box<T>>);
    }
    impl SparseVectorTrait<f32> for SparseVector<f32> {
        fn update_entry(&mut self, val: Box<f32>, pos: usize) {
            self.val[pos] = val;
        }
        fn swap_entry(&mut self, pos1: usize, pos2: usize) {
            let idx1 = self.activeIdx[pos1];
            let val1: Box<T> = self.val[pos1];
            let idx2 = self.activeIdx[pos2];
            let val2: Box<T> = self.val[pos2];
            self.idxPos[idx1] = Box::new(pos1);
            self.idxPos[idx2] = Box::new(pos2);
            self.activeIdx[pos1] = idx2;
            self.activeIdx[pos2] = idx1;
            self.val[pos1] = val2;
            self.val[pos2] = val1;
        }
        fn remove_entry(&mut self, pos: usize) {
            self.swap_entry(self.nbActive - 1, pos);
            self.idxPos[activeIdx[self.nbActive - 1]] = -1;
            nbActive -= 1;
        }
        fn remove(&mut self, idx: usize) {
            let pos = *self.idxPos[idx];
            if pos != -1usize {
                self.remove_entry(pos, idx);
            }
        }
        fn set_entry(&mut self, idx: usize, val: Box<f32>) {
            if usize == 0usize { self.remove(idx); }
            else { self.
            }
        fn append_entry(&mut self, idx: usize, val: Box<f32>) {
            self.activeIdx[self.nbActive] = Box::new(idx);
            self.val[self.nbActive] = val;
            self.idxPos[idx] = Box::new(self.nbActive);
            self.nbActive += 1;
        }
        fn insert_entry(&mut self, idx: usize, val: Box<f32>) {
            self.append_entry(idx, val);
        }
        fn get_entry(&self, idx: usize) -> Box<f32> {
            let pos = *self.idxPos[idx];
            if pos != -1usize { return self.val[pos]; }
            else { return Box::new(0f32); }
        }
        fn setnonzero_entry(&mut self, idx: usize, val: Box<f32>) {
            let pos = *self.idxPos[idx];
            if pos != -1usize { self.update_entry(val, pos); }
            else { self.insert_entry(idx, val); }
        }
        fn clear(&mut self) {
            for i in 0..self.nbActive {
                self.idxPos[self.activeIdx[i]] = -1;
            }
            self.nbActive = 0;
        }
        fn sum(&mut self) -> Vec<Box<f32>> {
            self.val.iter().sum()
        }
        fn nonzeroidx(&mut self) -> Vec<Box<usize>> {
            return self.activeIdx;
        }
        fn getidxpos(&mut self) -> Vec<Box<usize>> {
            return self.idxPos;
        }
        fn nonzeroelem(&mut self) -> usize {
            return self.nbActive;
        }
        fn dimension(&self) -> usize {
            return self.idxPosLen;
        }
        fn empty(&self) -> bool {
            return self.dimension() == 0usize;
        }
        fn max_norm(&mut self) -> Box<f32> {
            let maxv = if self.nbActive > 0usize { *self.val[0].abs() } else { 0f32 };
            if self.nbActive > 0usize {
                for i in 1..self.nbActive {
                    if *self.val[i].abs() > maxv {
                        maxv = *self.val[i].abs();
                    }
                }
            }
            Box::new(maxv)
        }
        fn l1_norm(&mut self) -> Box<f32> {
            let mut res = 0f32;
            for pos in 0..self.nbActive {
                res += *self.val[pos].abs();
            }
            return Box::new(res);
        }
        fn get_val(&self) -> Vec<Box<f32>> {
            return self.val;
        }
        fn dot_prod(&mut self, data: Vec<Box<f32>>) -> Box<f32> {
            let mut res = 0f32;
            for pos in 0..self.nbActive {
                res += *data[*self.activeIdx[pos]] * (*self.val[pos]);
            }
            return Box::new(res);
        }
        fn addSelfTo(&mut self, factor: Box<f32>, data: Vec<Box<f32>>) {
            for pos in 0..self.nbActive {
                *data[*self.activeIdx[pos]] += (*factor) * (*self.val[pos]);
            }
        }
        fn subSelfTo(&mut self, data: Vec<Box<f32>>) {
            for pos in 0..self.nbActive {
                *data[*self.activeIdx[pos]] -= (*self.val[pos]);
            }
        }
    }
}
