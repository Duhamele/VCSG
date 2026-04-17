
use num_traits::Float;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vector<const N:usize=3,T=f32> {
    pub data: [T; N],

}

impl<const N:usize,T> Vector<N,T> where T:Float
{
    pub fn new(tab: [T; N]) ->Self{
        Vector { data: tab }
    }
    pub fn norm(&self) -> T{
        let mut  somme:T=T::zero();
        for i in 0..N {
            somme=somme+self.data[i]*self.data[i];
        }
        T::sqrt(somme)
    }
    pub fn normalize(&mut self){
        let length = self.norm();
        for i in 0..N {
            self.data[i]= self.data[i]/length;
        }
    }
}
impl<const N:usize,T> std::ops::Add for Vector<N,T> where T:Float  {
    type Output = Vector<N,T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut point:[T;N] = [T::zero(); N];
        for i in 0..N {
            point[i] = self.data[i]+rhs.data[i];
        }
        Self::new(point)
    }
}
impl<const N:usize,T> std::ops::Sub for Vector<N,T> where T:Float {
    type Output = Vector<N,T>;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut point:[T;N] = [T::zero(); N];
        for i in 0..N {
            point[i] = self.data[i]-rhs.data[i];
        }
        Self::new(point)
    }
}
impl<const N:usize,T> std::ops::AddAssign for Vector<N,T> where T:Float{
    fn add_assign( &mut self, rhs: Self){
        *self = *self + rhs;
    }
}
impl<const N:usize,T> std::ops::SubAssign for Vector<N,T> where T:Float  {
    fn sub_assign(&mut self, rhs: Self) {
        *self=*self-rhs;
    }
}
impl <const N:usize,T> std::ops::MulAssign<T> for Vector<N,T> where T:Float {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.data[i]=rhs*self.data[i];
        }
    }
}
impl <const N:usize,T> Vector<N,T> where T:Float{
    pub fn zero() -> Self{
        Vector::new([T::zero(); N])
    }
    pub fn dot(&self, rhs: &Self) -> T {
        let mut sum:T=T::zero();
        for i in 0..N {
            sum=sum+self.data[i]*rhs.data[i];
        }
        sum
    }
    pub fn ones() -> Self{Vector::new([T::one(); N])}
}

impl <T> Vector<3,T> where T:Float{
    pub fn cross(&self, rhs: &Self) -> Vector<3,T> where T:Float{
        Vector::new([self.data[1]*rhs.data[2]-self.data[2]*rhs.data[1],
            self.data[2]*rhs.data[0]-self.data[0]*rhs.data[2],
            self.data[0]*rhs.data[1]-self.data[1]*rhs.data[0]])
    }
}

