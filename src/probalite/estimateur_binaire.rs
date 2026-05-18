use crate::probalite::estimator::Estimator;

///
/// Estimation de p
pub struct EstimatorSingle{
    nb_possible:u64,
    nb_value:u64,
}

impl EstimatorSingle {
    ///
    /// Initial de [EstimatorSingle]
    pub fn new() -> EstimatorSingle {
        EstimatorSingle{nb_possible:0,nb_value:0}
    }

    /// Ajout une mesure à l'estimation
    pub fn add_eval(&mut self, value: bool) {
        if value {
            self.nb_possible +=1;
        }
        self.nb_value += 1;
    }
    /// Ajout une mesure positif à l'estimation
    pub fn add_positif(&mut self){
        self.add_eval(true);
    }
    /// Ajout une mesure négatif à l'estimation
    pub fn add_negative(&mut self){
        self.add_eval(false);
    }

}
impl Estimator for EstimatorSingle {
    ///
    /// Retourne `true` si au moins une mesure enregistré sinon `false`.
    fn is_valid(&self) -> bool {
        self.nb_possible <= self.nb_value && self.nb_value>0
    }
    ///
    ///  Le retourne est toujours `true` sauf si les données sont corrompue.
    fn is_use(&self) -> bool {
        self.nb_possible <= self.nb_value
    }

    fn estimate(&self) -> Option<f64> {
        if self.nb_value==0{
            return None;
        }
        Some(self.nb_possible as f64 / self.nb_value as f64)
    }

    fn variance(&self) -> Option<f64> {
        if self.nb_value==0{
            return None;
        }
        let p=(self.nb_possible as f64 / self.nb_value as f64);
        Some(p*(1.-p))
    }
}



#[cfg(test)]
mod tests{




    use super::*;
    #[test]
    fn is_valid_test() {
        let mut estimator = EstimatorSingle::new();
        assert_eq!(estimator.is_valid(), false);
        estimator.add_eval(false);
        assert_eq!(estimator.is_valid(), true);
        estimator.add_positif();
        assert_eq!(estimator.is_valid(), true);
        estimator.add_negative();
        assert_eq!(estimator.is_valid(), true);

    }
}