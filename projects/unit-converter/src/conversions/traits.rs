pub trait UnitConversion: Sized {
    fn to_base(&self, value: f64) -> f64;

    fn from_base(&self, value: f64) -> f64;

    fn convert(&self, value: f64, to: &Self) -> f64 {
        let base_value = self.to_base(value);
        to.from_base(base_value)
    }
}
