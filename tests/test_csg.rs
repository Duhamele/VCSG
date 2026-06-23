use num_traits::abs;
use VCSG::csg;
use VCSG::csg::core_volu::VolumeCSG;
use VCSG::csg::point::Vector;
use VCSG::csg::random_generator::RandomGeneratorPointBox;
use VCSG::csg::volume::{Volume, VolumePrimaire};

#[test]
fn test_csg() {


    let boxe=csg::volume::Box::new(Vector::new([-1.,-1.,-1.]),Vector::new([2.,2.,2.]));
    let sphe=csg::volume::Sphere::new(Vector::new([-1.,0.,0.]), 1.);
    let volume=csg::operator::csg::Moins::new(Box::new(VolumeCSG::VolumePrimaire(VolumePrimaire::Box(boxe))),Box::new(VolumeCSG::VolumePrimaire(VolumePrimaire::Sphere(sphe))));
   let volu= volume.get_estimated_volume_single(1000000, &mut RandomGeneratorPointBox::new(csg::volume::Box::new(Vector::new([-1.,-1.,-1.]),Vector::new([2.,2.,2.]))));
    assert!(abs(volu-5.9)<0.1)
}