

///
/// Estimateur mono-dimensionnelle, le trait permet de lire les valeurs statiques.
/// Il ne permet pas utilisé pour évaluer. Il est utilisé après l'évaluation ou pendant
pub trait Estimator {
    /// Methods qui indique si l'estimateur peut donner les données statiques.
    ///
    /// Warning:
    ///
    /// [Estimator::is_valid] n'est pas égale à [Estimator::is_use].
    ///
    /// [Estimator::is_valid] ne valide pas [Estimator::is_use].
    fn is_valid(&self) -> bool;

    /// Méthodes qui indique si l'estimateur peut évaluer.
    ///
    /// Warning:
    ///
    ///  [Estimator::is_use] n'est pas égale à [Estimator::is_valid].
    /// 
    /// [Estimator::is_use] ne valide pas [Estimator::is_valid].
    fn is_use(&self) -> bool;
    ///
    /// Retourne l'estimation sous forme `Some<f64>` si possible sinon `Ǹone`.
    /// Si `Estimator::is_valid => false` alors c'est forcément `None`.
    /// 
    fn estimate(&self) -> Option<f64>;

    ///
    /// Retourne la variance sous forme `Some<f64>` si possible sinon `Ǹone`.
    /// Si `Estimator::is_valid => false` alors c'est forcément `None`.
    ///
    fn variance(&self) -> Option<f64>;
    
}
