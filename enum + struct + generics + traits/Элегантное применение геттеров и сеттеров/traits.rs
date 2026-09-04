pub trait Taxable {
    const TAX_RATE: f64;

    fn amount(&self) -> f64;

    fn set_amount(&mut self, new_amount: f64);

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }

    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0);
    }
}
