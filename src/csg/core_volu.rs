use num_traits::Float;
use crate::csg::operator::csg::{InterBinaire, Moins, UnionBinaire};
use crate::csg::point::Vector;
use crate::csg::volume::{Volume, VolumePrimaire};

#[derive( Debug)]
pub enum VolumeCSG<T:Float> {
    UnionBinaire(UnionBinaire<T>),
    VolumePrimaire(VolumePrimaire<T>),
    InterBinaire(InterBinaire<T>),
    Moins(Moins<T>),
    
    
    
    
}



impl <T:Float> Volume<T> for VolumeCSG<T> {
    fn is_in(self: &Self, point: &Vector<T>) -> bool {
        match self {
            VolumeCSG::UnionBinaire(volume) => {volume.is_in(point) },
            VolumeCSG::VolumePrimaire(volu ) => volu.is_in(point),
            VolumeCSG::InterBinaire(volu) => volu.is_in(point),
            VolumeCSG::Moins(volu) => volu.is_in(point),
        }
    }

    fn get_box_contains(self: &Self) -> crate::csg::volume::Box<T> {
        match self {
            VolumeCSG::UnionBinaire(v) => {v.get_box_contains() }
            VolumeCSG::VolumePrimaire(v) => {v.get_box_contains() }
            VolumeCSG::InterBinaire(v) => {v.get_box_contains() }
            VolumeCSG::Moins(v) => {v.get_box_contains() }
        }
    }

    fn get_volume(self: &Self) -> Option<T> {
        match self {
            VolumeCSG::UnionBinaire(v) => {v.get_volume()},
            VolumeCSG::VolumePrimaire(v) => {v.get_volume()}
            VolumeCSG::InterBinaire(v) => {v.get_volume()}
            VolumeCSG::Moins(v) => {v.get_volume()}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::csg::core_volu::{Volume, VolumeCSG};
    use crate::csg::core_volu::UnionBinaire;
    use crate::csg::point::Vector;
    use crate::csg::volume::{Sphere, VolumePrimaire};
    use crate::csg::volume::Box;

    #[test]
    fn test_csg_core_volu_union() {
        let sphere=Sphere::new(Vector::new([0.,0.,0.]),2.);
        let boite=Box::new(Vector::new([7.,0.,0.]),Vector::new([2.,1.,3.]));
        let volume=UnionBinaire::new(std::boxed::Box::new(VolumeCSG::VolumePrimaire(VolumePrimaire::Sphere(sphere))),std::boxed::Box::new(VolumeCSG::VolumePrimaire(VolumePrimaire::Box(boite))));
        let result=volume.get_volume().unwrap();
        assert!(f64::abs(result-39.5) < 0.1)
    }
}
