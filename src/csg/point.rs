use std::hash::{Hash, Hasher};
use std::io::Read;
use std::ptr::hash;
use num_traits::Float;

#[derive( Clone, Copy, Debug)]
pub struct Vector<T=f32> {
    pub data: [T; 3],

}

impl<T> Vector<T> where T:Float
{
    pub fn new(tab: [T; 3]) ->Self{
        Vector { data: tab }
    }
    pub fn norm(&self) -> T{
        let mut  somme:T=T::zero();
        for i in 0..3 {
            somme=somme+self.data[i]*self.data[i];
        }
        T::sqrt(somme)
    }
    pub fn normalize(&mut self){
        let length = self.norm();
        for i in 0..3{
            self.data[i]= self.data[i]/length;
        }
    }
}
impl<T> std::ops::Add for Vector<T> where T:Float  {
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut point:[T;3] = [T::zero(); 3];
        for i in 0..3 {
            point[i] = self.data[i]+rhs.data[i];
        }
        Self::new(point)
    }
}
impl<T> std::ops::Sub for Vector<T> where T:Float {
    type Output = Vector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut point:[T;3] = [T::zero(); 3];
        for i in 0..3 {
            point[i] = self.data[i]-rhs.data[i];
        }
        Self::new(point)
    }
}
impl<T> std::ops::AddAssign for Vector<T> where T:Float{
    fn add_assign( &mut self, rhs: Self){
        *self = *self + rhs;
    }
}
impl<T> std::ops::SubAssign for Vector<T> where T:Float  {
    fn sub_assign(&mut self, rhs: Self) {
        *self=*self-rhs;
    }
}
impl <T> std::ops::MulAssign<T> for Vector<T> where T:Float {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..3 {
            self.data[i]=rhs*self.data[i];
        }
    }
}
impl <T> Vector<T> where T:Float{
    pub fn zero() -> Self{
        Vector::new([T::zero(); 3])
    }
    pub fn dot(&self, rhs: &Self) -> T {
        let mut sum:T=T::zero();
        for i in 0..3{
            sum=sum+self.data[i]*rhs.data[i];
        }
        sum
    }
    pub fn ones() -> Self{Vector::new([T::one(); 3])}
}

impl <T> Vector<T> where T:Float{
    pub fn cross(&self, rhs: &Self) -> Vector<T> where T:Float{
        Vector::new([self.data[1]*rhs.data[2]-self.data[2]*rhs.data[1],
            self.data[2]*rhs.data[0]-self.data[0]*rhs.data[2],
            self.data[0]*rhs.data[1]-self.data[1]*rhs.data[0]])
    }
}

impl <T>  PartialEq<Self> for Vector<T> where T:Float{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..3 {
            if self.data[i].is_nan() ^ other.data[i].is_nan(){
                return false;
            }
            if self.data[i].is_zero() ^ other.data[i].is_zero(){
                return false;
            }
            if self.data[i].is_infinite() ^ other.data[i].is_nan() {
                return false;
            }
            if self.data[i].is_infinite(){
                if self.data[i].is_sign_negative() ^ self.data[i].is_sign_negative() {
                    return false;
                }
                continue;
            }
            if self.data[i]!=other.data[i]{
                return false;
            }
        }
        return true;
    }
}


impl <T> Eq for Vector<T> where T:Float {

}
trait FloatBits {
    type Bits:Hash;

    fn to_bits(self) -> Self::Bits;
}

impl FloatBits for f32 {
    type Bits = u32;

    fn to_bits(self) -> Self::Bits {
        f32::to_bits(self)
    }
}

impl FloatBits for f64 {
    type Bits = u64;

    fn to_bits(self) -> Self::Bits {
        f64::to_bits(self)
    }
}
impl <T> Hash for Vector<T> where T:Float  + FloatBits{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in 0..3 {
            if self.data[i].is_nan(){
                0u8.hash(state);
            }
            else if self.data[i].is_zero(){
                0u64.hash(state);
            }else {
                ( self.data[i].to_bits()).hash(state);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn add_vector() {
        let a = Vector::new([1.0, 2.0, 3.0]);
        let b = Vector::new([4.0, 5.0, 6.0]);
        let c = a + b;
        assert_eq!(c.data[0], 5.0);
        assert_eq!(c.data[1], 7.0);
        assert_eq!(c.data[2], 9.0);
    }

}
