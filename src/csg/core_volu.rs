use num_traits::Float;
use crate::csg::operator::csg::UnionBinaire;
use crate::csg::point::Vector;
use crate::csg::volume::{Volume, VolumePrimaire};

#[derive( Debug)]
pub enum VolumeCSG<T:Float> {
    UnionBinaire(UnionBinaire<T>),
    VolumePrimaire(VolumePrimaire<T>)
    
    
    
    
}



impl <T:Float> Volume<T> for VolumeCSG<T> {
    fn is_in(self: &Self, point: &Vector<T>) -> bool {
        match self { VolumeCSG::UnionBinaire(volume) => {volume.is_in(point) },
            &VolumeCSG::VolumePrimaire(volu ) => volu.is_in(point) }
    }

    fn get_box_contains(self: &Self) -> crate::csg::volume::Box<T> {
        match self {
            VolumeCSG::UnionBinaire(v) => {v.get_box_contains() }
            VolumeCSG::VolumePrimaire(v) => {v.get_box_contains() }
        }
    }
}
