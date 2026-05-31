


pub mod csg {
    use num_traits::Float;
    use crate::csg::core_volu::VolumeCSG;
    use crate::csg::operator::math::fussion;
    use crate::csg::point::Vector;
    use crate::csg::volume::Volume;

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
            None
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

    }
}