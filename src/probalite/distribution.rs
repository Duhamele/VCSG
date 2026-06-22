



use num_traits::Float;

pub fn distribution<T:Float>(fac1:T, fac2:T,toltal:u64) ->[u64;2] {
    if fac1.is_zero() { 
        return [0,toltal];
    }else if fac2.is_zero() {
        return [toltal,0];
    }else { 
        let factotal=fac1+fac2;
        let num1=((fac1/factotal)*T::from(toltal).unwrap()).to_i64().unwrap()as u64;
        let num2=toltal-num1;
        return [num1,num2];
    }
}