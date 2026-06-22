
use num_traits::{Float, FloatConst};
use rand::distr::{Distribution, StandardUniform};
use crate::csg::point::Vector;
use crate::csg::random_generator::RandomGeneratorPointBox;

pub trait Volume<T:Float> {

    //!
    //!
    //!
    //! @return if the point in the volume
    fn is_in(self: &Self, point: &Vector<T>) ->bool;



    fn get_box_contains(self: &Self)->Box<T>;

    ///
    /// @return Some If the volume can be determined mathematically, otherwise None.
    fn get_volume(self: &Self) -> Option<T>;



    ///
    fn get_estimated_volume_single(self: &Self,number_draw:u64, random_generator_point_box:&mut RandomGeneratorPointBox<T>) -> T where StandardUniform: Distribution<T>;



}

#[derive(Copy, Clone,Debug)]
pub struct Box<T> {
    pub base: Vector<T>,
    pub taille: Vector<T>,
}
impl<T:Float> Volume<T> for Box<T>{
    fn is_in(self: &Self, point: &Vector< T>) -> bool {
        for i in 0..3{
            if point.data[i]< self.base.data[i] || (self.base.data[i]+self.taille.data[i]) < point.data[i] {
                return false;
            }
        }
        true
    }

    fn get_box_contains(self: &Self) -> Box< T> {
        self.clone()
    }

    fn get_volume(self: &Self) -> Option<T> {
        Some(self.taille.data[0]*self.taille.data[1]*self.taille.data[2])
    }

    fn get_estimated_volume_single(self: &Self, number_draw: u64, random_generator_point_box: &mut RandomGeneratorPointBox<T>) -> T where StandardUniform: Distribution<T>{
        self.taille.data[0]*self.taille.data[1]*self.taille.data[2]
    }
}
impl<T:Float> Box<T>{
    pub fn new(base: Vector< T>, taille: Vector< T>) -> Self {
        Self{base, taille}
    }
    pub fn cal_volume(self:&Self)->T{
        self.taille.data[0]*self.taille.data[1]*self.taille.data[2]
    }
    pub fn is_null(self: &Self) -> bool{
        self.taille.data[0].is_zero() || self.taille.data[1].is_zero() || self.taille.data[2].is_zero()
    }
}
impl <T:Float> PartialEq for Box<T>{
    fn eq(self: &Self, other: &Self) -> bool {
        self.base == other.base && self.taille == other.taille
    }
}

#[derive(Copy, Clone,Debug)]
pub struct Sphere<T:Float> {
    pub center: Vector<T>,
    pub radius:T,
}

impl<T:Float> Volume<T> for Sphere<T> {
    fn is_in(self: &Self, point: &Vector< T>) -> bool {
        (self.center- *point).norm()<self.radius
    }

    fn get_box_contains(self: &Self) -> Box< T> {
        let mut base=self.center.clone();
        for i in 0..3{
            base.data[i] = base.data[i]-self.radius;
        }
        let mut taille=Vector::ones();
        taille*= self.radius;
        Box::new(base,taille)

    }

    fn get_volume(self: &Self) -> Option<T> {
        Some(T::from(4./3.).unwrap()*T::powi(self.radius,3)*T::from(f64::PI()).unwrap())
    }

    fn get_estimated_volume_single(self: &Self, number_draw: u64, random_generator_point_box: &mut RandomGeneratorPointBox<T>) -> T where StandardUniform: Distribution<T> {
        self.cal_volume()
    }
}
impl <T:Float> Sphere<T> {
    pub fn new(center: Vector<T>, radius:T) -> Self {
        Self{center, radius}
    }
    pub fn cal_volume(self: &Self)->T{
        T::powi(self.radius,3)*T::from(f64::PI()*4.0f64/3.0f64).unwrap()
    }
    

}
#[derive(Copy, Clone,Debug)]
pub struct Tore<T:Float>{
    pub center: Vector<T>,
    pub axe: Vector<T>,
    pub radius_inter:T,
    pub radius_larger:T,
    pub radius_height:T,
}
impl<T:Float> Tore<T>{

    pub fn new(center:&Vector<T>, axe:&Vector<T>,radius_inter:T,radius_larger:T,
              radius_height:T,) -> Self{
        let mut axe = axe.clone();
        axe.normalize();
        Self{
          center: *center,axe,radius_inter,radius_larger,radius_height
        }
    }
    pub fn is_empty_center(&self)->bool{
        self.radius_inter>self.radius_larger
    }

}
impl<T:Float> Volume<T> for Tore<T>{
    fn is_in(self: &Self, point: &Vector< T>) -> bool {
        let d_origin =  *point-self.center;
        let height= self.axe.dot(&d_origin);
        let distance_axe=d_origin.cross(&self.axe).norm();
        let distance_cercle=distance_axe-self.radius_inter;
        let norme_cercle=(height*height)/(self.radius_height*self.radius_height) +(distance_cercle*distance_cercle)/(self.radius_larger*self.radius_larger);
        norme_cercle <=T::one()
    }

    fn get_box_contains(self: &Self) -> Box< T> {
        let cote2=self.radius_inter+T::max(self.radius_larger, self.radius_height);
        let base=self.center - Vector::new([cote2;3]);
        let taille= Vector::new([cote2;3])+Vector::new([cote2;3]);
        Box::new(base,taille)

    }

    fn get_volume(self: &Self) -> Option<T> {
        if self.radius_inter>=self.radius_larger {
            return Some(self.radius_larger*self.radius_height*self.radius_inter*T::from(2.0f64*f64::PI()*f64::PI()).unwrap());
        }

        Some(T::from(2.0f64*f64::PI()*f64::PI()).unwrap()*self.radius_larger*self.radius_height*self.radius_inter+T::from((4.0/3.0)*f64::PI()).unwrap()*(self.radius_height/self.radius_larger)*(self.radius_larger-self.radius_inter)*(self.radius_larger-self.radius_inter)*(self.radius_larger+T::from(2.).unwrap()*self.radius_inter))

    }

    fn get_estimated_volume_single(self: &Self, number_draw: u64, random_generator_point_box: &mut RandomGeneratorPointBox<T>) -> T where StandardUniform: Distribution<T>{
        self.get_volume().unwrap()
    }
}


#[derive(Copy, Clone,Debug)]
pub enum VolumePrimaire<T:Float>{
    Box(Box<T>),
    Sphere(Sphere< T>),
    Tore(Tore<T>),
}
impl <T:Float> Volume<T> for VolumePrimaire<T> where {
    fn is_in(self: &Self, point: &Vector< T>) -> bool {
        match self {
            VolumePrimaire::Box(volume) => {volume.is_in(point)},
            VolumePrimaire::Sphere(volume) => {volume.is_in(point)},
            VolumePrimaire::Tore(volume) => {volume.is_in(point)},
        }
    }

    fn get_box_contains(self: &Self) -> Box< T> {
        match self {
            VolumePrimaire::Box(volume) => {volume.get_box_contains()}
            VolumePrimaire::Sphere(volume) => {volume.get_box_contains()}
            VolumePrimaire::Tore(volume) => {volume.get_box_contains()}
        }
    }

    fn get_volume(self: &Self) -> Option<T> {
        match self {
            VolumePrimaire::Box(volume) => {volume.get_volume()},
            VolumePrimaire::Sphere(volume) => {volume.get_volume()},
            VolumePrimaire::Tore(volume) => {volume.get_volume()},
        }
    }

    fn get_estimated_volume_single(self: &Self, number_draw: u64, random_generator_point_box: &mut RandomGeneratorPointBox<T>) -> T where StandardUniform: Distribution<T> {
        match self {
            VolumePrimaire::Box(volume) => {volume.get_estimated_volume_single(number_draw, random_generator_point_box)},
            VolumePrimaire::Sphere(volume) => {volume.get_estimated_volume_single(number_draw, random_generator_point_box)},
            VolumePrimaire::Tore(volume) => {volume.get_estimated_volume_single(number_draw, random_generator_point_box)},
        }
    }
}



#[cfg(test)]
mod tests{
    use crate::csg::volume::{Tore, Volume};
use crate::csg::point::Vector;


    #[test]
    fn test_tore_point(){
       let tore=Tore::new(&Vector::ones(),&Vector::ones(),1.,1.,1.);
        assert!(tore.is_in(&Vector::ones()));

    }
}