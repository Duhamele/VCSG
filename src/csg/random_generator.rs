use num_traits::{ Float};
use crate::csg::point::{Vector};
use crate::csg::volume::Box;
use rand::{ RngExt};
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::distr::StandardUniform;
use rand::prelude::Distribution;

#[derive( Debug)]
pub struct RandomGeneratorPointBox<T:Float> {
    start:Vector<T>,
    end:Vector<T>,
    core_rand:rand::rngs::StdRng,

}
impl <T:Float> RandomGeneratorPointBox<T> where StandardUniform: Distribution<T>  {



    pub fn new(_box:Box<T>) -> Self {

        Self{start:_box.base,end:_box.base+_box.taille,core_rand: StdRng::seed_from_u64(42)}

    }
    pub fn set_seed(&mut self, seed:u64) {
        self.core_rand=StdRng::seed_from_u64(seed);

    }
    pub fn draw(&mut self)->Vector<T>{
        let mut output:Vector<T>=Vector::zero();
        for i in 0..3{
            output.data[i]=self.core_rand.random::<T>()*(self.end.data[i]-self.start.data[i])+self.start.data[i];
        }




        output
    }
}


#[cfg(test)]
mod tests{
    use crate::csg::volume::Volume;
    use super::*;

    #[test]
    fn test_random_generator_point() {
        use crate::csg::volume::Box;
        let volume=Box::new(Vector::< f32>::ones(),Vector::ones());
        let mut random=RandomGeneratorPointBox::new(volume);
        for _ in 0..100 {
            assert!(volume.is_in(&random.draw()))
        }
    }
    #[test]
    fn test_random_generator_no_loop(){
        use crate::csg::volume::Box;
        let volume=Box::new(Vector::< f32>::ones(),Vector::ones());
        let mut random=RandomGeneratorPointBox::new(volume);
        let mut set_=std::collections::HashSet::new();
        for _ in 0..100000 {
            let point_tmp=random.draw();
            assert!(!set_.contains(&point_tmp));
            set_.insert(point_tmp);

        }
    }
    #[test]
    fn test_random_seed(){
        let volume=Box::new(Vector::< f32>::ones(),Vector::ones());
        let mut random=RandomGeneratorPointBox::new(volume);
        let mut random2=RandomGeneratorPointBox::new(volume);
        random.set_seed(54);
        assert_ne!(random2.draw(), random.draw());
    }
}