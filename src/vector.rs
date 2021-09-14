mod rllib {
    pub enum VectorType {
        BaseVector,
        DenseVector,
        SparseVector,
    }
    
    pub struct Vector {
        vectortype: VectorType,
    }
    impl Vector {
        fn new(&self, vt: VectorType) -> Self {
            Self {
                vectortype: vt,
            }
        }
    }
    pub trait VectorTrait<T> {
        fn dimension(&self) -> usize;
        fn empty(&self) -> bool;
        fn max_norm(&self) -> T;
        fn l1_norm(&self) -> T;
        //fn l2_norm(&self) -> T;
        fn sum(&self) -> T;
        fn get_values(&self) -> Vec<T>;
        fn get_entry(&self, idx: usize) -> T;
        //fn dot(&mut self, other: Vector<T>) -> T;
        fn get(&self, idx: usize) -> T;
        fn clear(&mut self);
        fn set_entry(&mut self, idx: usize, val: T);
        fn remove_entry(&mut self, idx: usize);
        fn add_toself(&mut self, val: T) -> Vec<T>;
        //fn substractToself(&mut self, other: Vec<T>) -> Vec<T>;
        fn map_multiply_toself(&mut self, factor: T) -> Vec<T>;
        //fn set(&mut self, other: Vec<T>) -> Vec<T>;
        //fn set(&mut self, other: Vec<T>, offset: i32) -> Vec<T>;
        //deep copy of vector
        //fn copy(&mut self) -> Box<Vector<T>>;
        //fn newInstance(&mut self, dim: i32) -> Box<Vector<T>>;
        //storage management
        //fn persist(&mut self, f: String);
        //fn resurrect(&mut self, f: String);
    }
    /*
    pub trait VectorTraitaddToself1<T> {
        fn addToself(&mut self, factor: T, other: Box<Vector>) -> Box<Vector>;
    }
    pub trait VectorTraitaddToself2<T> {
        fn addToself(&mut self, other: Box<Vector>) -> Box<Vector>;
    }
    */
    pub trait Type {
        fn abs(&self) -> Self;
    }
    impl Type for f32 {
        fn abs(&self) -> Self {
            Self::abs(*self)
        }
    }
    pub struct DenseVector<T>
    where 
        T: Copy + Clone,
    {
        capacity: usize,
        data: Vec<T>,
    }
    impl DenseVector<f32> {
        fn new(capacity: usize, data: Vec<T>) -> Self {
            Self {
                capacity: 1usize,
                data: data.clone(),
            }
        }
    }
    /*
    impl<f32> Index<usize> for DenseVector<f32> {
        type Output = f32;
        fn index(&self, idx: usize) -> &f32 {
            &self.get(idx)
        }
    }
    */

    impl VectorTrait<f32> for DenseVector<f32> {
        fn dimension(&self) -> usize {
            return self.capacity;
        }
        fn empty(&self) -> bool {
            return self.dimension() == 0usize;
        }
        fn max_norm(&self) -> f32 {
            let mut maxv = self.data[0];
            if self.capacity > 0usize {
                for i in 1..self.capacity {
                    if self.data[i].abs() > maxv {
                        maxv = self.data[i].abs();
                    }
                }
            }
            return maxv;
        }
        fn l1_norm(&self) -> f32 {
            let mut result = self.data[0];
            for i in 1..self.capacity {
                result += self.data[i].abs();
            }
            return result;
        }
        fn sum(&self) -> f32 {
            self.data.iter().sum()
        }
        fn get_values(&self) -> Vec<f32> {
            return self.data.clone();
        }
        fn get_entry(&self, idx: usize) -> f32 {
            return self.data[idx];
        }
        fn clear(&mut self) {
            self.data.clear();
        }
        fn set_entry(&mut self, idx: usize, val: f32) {
            self.data[idx] = val;
        }
        fn get(&self, idx: usize) -> f32 {
            return self.data[idx];
        }
        fn remove_entry(&mut self, idx: usize) {
            self.data.remove(idx);
        }
        fn add_toself(&mut self, val: f32) -> Vec<f32> {
            for i in 0..self.capacity {
                self.data[i] += val;
            }
            self.data.clone()
        }
        fn map_multiply_toself(&mut self, factor: f32) -> Vec<f32> {
            for i in 0..self.capacity {
                self.data[i] *= factor;
            }
            self.data.clone()
        }
        
    }
    pub struct SparseVector<T> {
        idx_pos_len: usize,
        active_idx_len: usize,
        nb_active: usize,
        idx_pos: Vec<usize>,
        active_idx: Vec<usize>,
        val: Vec<T>,
    }
    impl<T> SparseVector<T> {
        fn new(capacity: usize, active_idx: usize, 
        active_idx_len: usize, ) -> Self {
            Self {
                idx_pos_len: capacity,
                active_idx_len: active_idx_len,
                nb_active: active_idx,
                idx_pos: vec![],
                active_idx: vec![],
                val: vec![],
            }
        }
    }
    pub trait SparseVectorTrait<T> {
        fn update_entry(&mut self, val: T, pos: usize);
        fn swap_entry(&mut self, pos1: usize, pos2: usize);
        fn remove_entry(&mut self, pos: usize);
        fn remove(&mut self, idx: usize);
        fn set_entry(&mut self, idx: usize, val: T);
        fn append_entry(&mut self, idx: usize, val: T);
        fn insert_entry(&mut self, idx: usize, val: T);
        fn get_entry(&self, idx: usize) -> T;
        fn setnonzero_entry(&mut self, idx: usize, val: T);
        fn clear(&mut self);
        fn sum(&mut self) -> T;
        fn nonzeroidx(&mut self) -> Vec<usize>;
        fn getidxpos(&mut self) -> Vec<usize>;
        fn nonzeroelem(&mut self) -> usize;
        fn dimension(&self) -> usize;
        fn empty(&self) -> bool;
        fn max_norm(&mut self) -> T;
        fn l1_norm(&mut self) -> T;
        fn get_val(&self) -> Vec<T>;
        fn dot_prod(&mut self, data: Vec<T>) -> T;
        fn add_self_to(&mut self, factor: T, data: Vec<T>);
        fn sub_self_to(&mut self, data: Vec<T>);
    }
    impl SparseVectorTrait<f32> for SparseVector<f32> {
        fn update_entry(&mut self, val: f32, pos: usize) {
            self.val[pos] = val;
        }
        fn swap_entry(&mut self, pos1: usize, pos2: usize) {
            let idx1 = self.active_idx[pos1];
            let val1 = self.val[pos1];
            let idx2 = self.active_idx[pos2];
            let val2 = self.val[pos2];
            self.idx_pos[idx1] = pos1;
            self.idx_pos[idx2] = pos2;
            self.active_idx[pos1] = idx2;
            self.active_idx[pos2] = idx1;
            self.val[pos1] = val2;
            self.val[pos2] = val1;
        }
        fn remove_entry(&mut self, pos: usize) {
            self.swap_entry(self.nb_active - 1, pos);
            self.idx_pos[self.active_idx[self.nb_active - 1]] = usize::MAX;
            self.nb_active -= 1;
        }
        fn remove(&mut self, idx: usize) {
            let pos = self.idx_pos[idx];
            if pos != usize::MAX {
                self.remove_entry(idx);
            }
        }
        fn set_entry(&mut self, idx: usize, val: f32) {
            if val == 0f32 { self.remove(idx); }
            else { self.setnonzero_entry(idx, val); }
        }
        fn append_entry(&mut self, idx: usize, val: f32) {
            self.active_idx[self.nb_active] = idx;
            self.val[self.nb_active] = val;
            self.idx_pos[idx] = self.nb_active;
            self.nb_active += 1;
        }
        fn insert_entry(&mut self, idx: usize, val: f32) {
            self.append_entry(idx, val);
        }
        fn get_entry(&self, idx: usize) -> f32 {
            let pos = self.idx_pos[idx];
            if pos != usize::MAX { return self.val[pos]; }
            else { return 0f32; }
        }
        fn setnonzero_entry(&mut self, idx: usize, val: f32) {
            let pos = self.idx_pos[idx];
            if pos != usize::MAX { self.update_entry(val, pos); }
            else { self.insert_entry(idx, val); }
        }
        fn clear(&mut self) {
            for i in 0..self.nb_active {
                self.idx_pos[self.active_idx[i]] = usize::MAX;
            }
            self.nb_active = 0;
        }
        fn sum(&mut self) -> f32 {
            self.val.iter().sum()
        }
        fn nonzeroidx(&mut self) -> Vec<usize> {
            return self.active_idx.clone();
        }
        fn getidxpos(&mut self) -> Vec<usize> {
            return self.idx_pos.clone();
        }
        fn nonzeroelem(&mut self) -> usize {
            return self.nb_active;
        }
        fn dimension(&self) -> usize {
            return self.idx_pos_len;
        }
        fn empty(&self) -> bool {
            return self.dimension() == 0usize;
        }
        fn max_norm(&mut self) -> f32 {
            let mut maxv = if self.nb_active > 0usize { self.val[0].abs() } else { 0f32 };
            if self.nb_active > 0usize {
                for i in 1..self.nb_active {
                    if self.val[i].abs() > maxv {
                        maxv = self.val[i].abs();
                    }
                }
            }
            maxv
        }
        fn l1_norm(&mut self) -> f32 {
            let mut res = 0f32;
            for pos in 0..self.nb_active {
                res += self.val[pos].abs();
            }
            return res;
        }
        fn get_val(&self) -> Vec<f32> {
            return self.val.clone();
        }
        fn dot_prod(&mut self, data: Vec<f32>) -> f32 {
            let mut res = 0f32;
            for pos in 0..self.nb_active {
                res += data[self.active_idx[pos]] * self.val[pos];
            }
            return res;
        }
        fn add_self_to(&mut self, factor: f32, mut data: Vec<f32>) {
            for pos in 0..self.nb_active {
                data[self.active_idx[pos]] += factor * self.val[pos];
            }
        }
        fn sub_self_to(&mut self, mut data: Vec<f32>) {
            for pos in 0..self.nb_active {
                data[self.active_idx[pos]] -= self.val[pos];
            }
        }
    }
}
