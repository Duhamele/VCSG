


pub mod csg {
    use num_traits::Float;
    use rand::distr::{Distribution, StandardUniform};
    use crate::csg::core_volu::VolumeCSG;
    use crate::csg::operator::math::{fussion, inter};
    use crate::csg::point::Vector;
    use crate::csg::random_generator::RandomGeneratorPointBox;
    use crate::csg::volume::Volume;
    use crate::probalite::distribution::distribution;

    #[derive( Debug)]
    pub struct UnionBinaire<T:Float> {
        volumes: [Box<VolumeCSG<T> >;2]
    }
    impl<T:Float>  UnionBinaire<T>  {
        pub fn new(volume1:Box<VolumeCSG<T> >,volumes2:Box<VolumeCSG<T> >) -> UnionBinaire<T>  {
            UnionBinaire::<T>{volumes:[volume1,volumes2]}
        }

    }
    impl <T:Float> Volume<T> for UnionBinaire<T>  {
        fn is_in(self: &Self, point: &Vector<T>) -> bool {
            self.volumes[0].is_in(point)||self.volumes[1].is_in(point)
        }

        fn get_box_contains(self: &Self) -> crate::csg::volume::Box<T> {
            fussion::<T>(self.volumes[0].get_box_contains(),self.volumes[1].get_box_contains())
        }

        fn get_volume(self: &Self) -> Option<T> {
            let boxes=inter::<T>(self.volumes[0].get_box_contains(),self.volumes[1].get_box_contains());
            if boxes.cal_volume().is_zero() {
                let volu1=self.volumes[1].get_volume();
                let volu0=self.volumes[0].get_volume();
                if volu0.is_some()& volu1.is_some() {
                    return Some(volu0.unwrap()+volu1.unwrap());
                }
            }
            None
        }

        fn get_estimated_volume_single(self: &Self, number_draw: u64, random_generator_point_box: &mut RandomGeneratorPointBox<T>) -> T where StandardUniform: Distribution<T>{
            let box1=self.volumes[0].get_box_contains();
            let box2=self.volumes[1].get_box_contains();
            if inter(box1,box2).is_null() {
                if box1.is_null() {
                    return self.volumes[1].get_estimated_volume_single(number_draw, random_generator_point_box);
                }else if box2.is_null() {
                    return self.volumes[0].get_estimated_volume_single(number_draw, random_generator_point_box);
                }else {
                    let fact1=box1.cal_volume();
                    let fact2=box2.cal_volume();
                    let fact_i =distribution(fact1, fact2, number_draw);
                    return self.volumes[0].get_estimated_volume_single(fact_i[0], random_generator_point_box)+self.volumes[1].get_estimated_volume_single(fact_i[1], random_generator_point_box);
                }
            }else {
                let mut sum=0;
                random_generator_point_box.set(fussion(box1,box2));
                for _ in 0..number_draw {
                    if self.is_in(&random_generator_point_box.draw()) {
                        sum+=1;
                    }
                }
                return fussion(box1,box2).cal_volume()*(T::from(sum).unwrap()/T::from(number_draw).unwrap());
            }
        }
    }
    #[derive( Debug)]
    pub struct InterBinaire<T:Float> {
        volumes: [Box<VolumeCSG<T> >;2]
    }
    impl<T:Float>  InterBinaire<T>  {
        pub fn new(volume1:Box<VolumeCSG<T> >,volumes2:Box<VolumeCSG<T> >) ->InterBinaire<T>{
            InterBinaire{volumes:[volume1,volumes2]}
        }
    }
    impl <T:Float> Volume<T> for InterBinaire<T>  {
        fn is_in(self: &Self, point: &Vector<T>) -> bool {
            self.volumes[0].is_in(point) && self.volumes[1].is_in(point)
        }

        fn get_box_contains(self: &Self) -> crate::csg::volume::Box<T> {
            inter(self.volumes[1].get_box_contains(),self.volumes[0].get_box_contains())
        }

        fn get_volume(self: &Self) -> Option<T> {
            if self.get_box_contains().cal_volume().is_zero() {
                return Some(T::zero());
            }
            None
        }

        fn get_estimated_volume_single(self: &Self, number_draw: u64, random_generator_point_box: &mut RandomGeneratorPointBox<T>) -> T
        where
            StandardUniform: Distribution<T>
        {
            let box1=self.volumes[0].get_box_contains();
            let box2=self.volumes[1].get_box_contains();
            if inter(box1,box2).is_null() {
                return T::zero();
            }else {
                random_generator_point_box.set(inter(box1,box2));
                let mut sum=0;
                for _ in 0..number_draw {
                    if self.is_in(&random_generator_point_box.draw()) {
                        sum+=1;
                    }
                }
                return inter(box1,box2).cal_volume()*(T::from(sum).unwrap()/T::from(number_draw).unwrap());

            }
        }
    }
    #[derive( Debug)]
    pub struct Moins<T:Float> {
        volumes_base:Box<VolumeCSG<T> >,
        volume_moins:Box<VolumeCSG<T> >
    }
    impl<T:Float>  Moins<T>  {
        pub fn new(volumes_base:Box<VolumeCSG<T> >,
                   volume_moins:Box<VolumeCSG<T> >)-> Moins<T>  {
            Self{volumes_base,volume_moins}
        }

    }
    impl <T:Float> Volume<T> for Moins<T>  {
        fn is_in(self: &Self, point: &Vector<T>) -> bool {
            self.volumes_base.is_in(point) && !self.volume_moins.is_in(point)
        }

        fn get_box_contains(self: &Self) -> crate::csg::volume::Box<T> {
            self.volumes_base.get_box_contains()
        }

        fn get_volume(self: &Self) -> Option<T> {
            if inter(self.volumes_base.get_box_contains(),self.volume_moins.get_box_contains()).cal_volume().is_zero() {
                return self.volumes_base.get_volume();
            }
            None
        }

        fn get_estimated_volume_single(self: &Self, number_draw: u64, random_generator_point_box: &mut RandomGeneratorPointBox<T>) -> T
        where
            StandardUniform: Distribution<T>
        {
            let box1=self.volumes_base.get_box_contains();
            let box2=self.volume_moins.get_box_contains();
            if inter(box1,box2).is_null() {
                return self.volumes_base.get_estimated_volume_single(number_draw, random_generator_point_box);
            }else {
                random_generator_point_box.set(fussion(box1,box2));
                let mut sum=0;
                for _ in 0..number_draw {
                    if self.is_in(&random_generator_point_box.draw()) {
                        sum+=1;
                    }
                }
                return fussion(box1,box2).cal_volume()*(T::from(sum).unwrap()/T::from(number_draw).unwrap());

            }
        }
    }

}

pub mod math {
    use num_traits::Float;
    use crate::csg::point::Vector;

    pub fn fussion<T:Float>(box1:crate::csg::volume::Box<T>, box2:crate::csg::volume::Box<T>) ->crate::csg::volume::Box<T> {
        let mut base:[T;3]=[T::one(),T::one(),T::one()];
        let mut sommet:[T;3]=[T::one(),T::one(),T::one()];
        for i in 0..3 {
            base[i]=T::min(box1.base.data[i],box2.base.data[i]);
            sommet[i]=T::max(box1.base.data[i]+box1.taille.data[i],box2.base.data[i]+box2.taille.data[i])-base[i];
        }
        crate::csg::volume::Box::new(Vector::new(base),Vector::new(sommet))

    }

    pub fn inter<T:Float>(box1:crate::csg::volume::Box<T>, box2:crate::csg::volume::Box<T>)->crate::csg::volume::Box<T>{
        let mut base:[T;3]=[T::one(),T::one(),T::one()];
        let mut sommet:[T;3]=[T::one(),T::one(),T::one()];
        for i in 0..3 {
            base[i]=T::max(box1.base.data[i],box2.base.data[i]);
            sommet[i]=T::max(T::min(box1.base.data[i]+box1.taille.data[i],box2.base.data[i]+box2.taille.data[i])-base[i],T::zero());
        }
        crate::csg::volume::Box::new(Vector::new(base),Vector::new(sommet))
    }
    #[cfg(test)]
    mod test{

        use crate::csg::operator::math::fussion;
        use crate::csg::point::Vector;



        #[test]
        fn test_fussion() {
            let a=crate::csg::volume::Box::new(Vector::new([1.0, 0.0, 0.0]),Vector::new([1., 1.0, 1.0]));
            let b=crate::csg::volume::Box::new(Vector::new([0.0, 0.0, 0.0]),Vector::new([1., 1.0, 1.0]));
            let c=crate::csg::volume::Box::new(Vector::new([0.0, 0.0, 0.0]),Vector::new([2., 1.0, 1.0]));
            let c2=fussion(a,b);
            assert_eq!(c, c2)
        }

        use rstest::rstest;
        #[rstest]
        #[case([1.0, 0.0, 0.0],[1., 1.0, 1.0],[0.0, 0.0, 0.0],[1., 1.0, 1.0],[0.0, 0.0, 0.0],[2., 1.0, 1.0])]
        #[case([0.0, 0.0, 0.0],[1., 1.0, 1.0],[2.0, 2.0, 2.0],[1., 1.0, 1.0],[0.0, 0.0, 0.0],[3., 3.0, 3.0])]
        #[case([0.0, 1.0, 0.0],[1., 1.0, 1.0],[0.0, 0.0, 0.0],[1., 1.0, 1.0],[0.0, 0.0, 0.0],[1., 2.0, 1.0])]
        #[case([0.0, 0.0, 1.0],[1., 1.0, 1.0],[0.0, 0.0, 0.0],[1., 1.0, 1.0],[0.0, 0.0, 0.0],[1., 1.0, 2.0])]
        fn test_fussion_serie(#[case] a_b:[f32;3],#[case] a_t:[f32;3], #[case] b_b: [f32;3],#[case] b_t:[f32;3], #[case] c_b: [f32;3],#[case] c_t:[f32;3]) {
            let a=crate::csg::volume::Box::new(Vector::new(a_b),Vector::new(a_t));
            let b=crate::csg::volume::Box::new(Vector::new(b_b),Vector::new(b_t));
            let c=fussion(a,b);
            let c2=crate::csg::volume::Box::new(Vector::new(c_b),Vector::new(c_t));
            assert_eq!(c, c2)

        }

        

    }
}