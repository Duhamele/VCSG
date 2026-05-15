use num_traits::{zero, Float};
use crate::csg::point::{Vector};
use crate::csg::volume::Box;
use rand::{Rng, RngExt};
use rand::distr::StandardUniform;
use rand::prelude::Distribution;

#[derive( Clone,Debug)]
pub struct RandomGeneratorPointBox<const N: usize,T:Float> {
    start:Vector<N,T>,
    end:Vector<N,T>,
    core_rand:rand::rngs::ThreadRng,

}
impl <const N:usize,T:Float> RandomGeneratorPointBox<N,T> where StandardUniform: Distribution<T>  {



    pub fn new(_box:Box<N,T>) -> Self {

        Self{start:_box.base,end:_box.base+_box.taille,core_rand: rand::rng()}

    }
    pub fn set_seed(&mut self, seed:u64) {
        todo!()
    }
    pub fn draw(&mut self)->Vector<N,T>{
        let mut output:Vector<N,T>=Vector::zero();
        for i in 0..N{
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
        let volume=Box::new(Vector::<3, f32>::ones(),Vector::ones());
        let mut random=RandomGeneratorPointBox::new(volume);
        for _ in 0..100 {
            assert!(volume.is_in(&random.draw()))
        }
    }
    #[test]
    fn test_random_generator_no_loop(){
        use crate::csg::volume::Box;
        let volume=Box::new(Vector::<3, f32>::ones(),Vector::ones());
        let mut random=RandomGeneratorPointBox::new(volume);
        let mut set_=std::collections::HashSet::new();
        for _ in 0..100000 {
            let point_tmp=random.draw();
            assert!(!set_.contains(&point_tmp));
            set_.insert(point_tmp);

        }
    }
}