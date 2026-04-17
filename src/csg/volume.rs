use num_traits::Float;
use crate::csg::point::Vector;

pub trait Volume<const N:usize,T:Float> {

    //!
    //!
    //!
    //! @return if the point in the volume
    fn is_in(self: &Self, point: &Vector<N,T>) ->bool;



    fn get_box_contains(self: &Self)->Box<N,T>;






}

#[derive(Copy, Clone,Debug)]
pub struct Box<const N:usize,T> {
    pub base: Vector<N,T>,
    pub taille: Vector<N,T>,
}
impl<const N:usize,T:Float> Volume<N,T> for Box<N,T>{
    fn is_in(self: &Self, point: &Vector<N, T>) -> bool {
        for i in 0..N{
            if point.data[i]< self.base.data[i] || (self.base.data[i]+self.taille.data[i]) < point.data[i] {
                return false;
            }
        }
        true
    }

    fn get_box_contains(self: &Self) -> Box<N, T> {
        self.clone()
    }
}
impl<const N:usize,T:Float> Box<N,T>{
    pub fn new(base: Vector<N, T>, taille: Vector<N, T>) -> Self {
        Self{base, taille}
    }
}

#[derive(Copy, Clone,Debug)]
pub struct Sphere<const N:usize,T:Float> {
    pub center: Vector<N,T>,
    pub radius:T,
}

impl<const N:usize,T:Float> Volume<N,T> for Sphere<N,T> {
    fn is_in(self: &Self, point: &Vector<N, T>) -> bool {
        (self.center- *point).norm()<self.radius
    }

    fn get_box_contains(self: &Self) -> Box<N, T> {
        let mut base=self.center.clone();
        for i in 0..N{
            base.data[i] = base.data[i]-self.radius;
        }
        let mut taille=Vector::ones();
        taille*= self.radius;
        Box::new(base,taille)

    }
}

#[derive(Copy, Clone,Debug)]
pub struct Tore<T:Float>{
    pub center: Vector<3,T>,
    pub axe: Vector<3,T>,
    pub radius_inter:T,
    pub radius_larger:T,
    pub radius_height:T,
}
impl<T:Float> Volume<3,T> for Tore<T>{
    fn is_in(self: &Self, point: &Vector<3, T>) -> bool {
        let d_origin =  *point-self.center;
        let height= self.axe.dot(&d_origin);
        let distance_axe=d_origin.cross(&self.axe).norm();
        let distance_cercle=distance_axe-self.radius_inter;
        let norme_cercle=(height*height)/(self.radius_height*self.radius_height) +(distance_cercle*distance_cercle)/(self.radius_larger*self.radius_larger);
        norme_cercle <=T::one()
    }

    fn get_box_contains(self: &Self) -> Box<3, T> {
        let cote2=self.radius_inter+T::max(self.radius_larger, self.radius_height);
        let base=self.center - Vector::new([cote2;3]);
        let taille= Vector::new([cote2;3])+Vector::new([cote2;3]);
        Box::new(base,taille)

    }
}



pub enum VolumePrimaire<const N:usize,T:Float>{
    Box(Box<N,T>),
    Sphere(Sphere<N, T>),
    Tore(Tore<T>),
}