use crate::traits::Taxable;

pub struct Bonus {
    pub amount: f64,
}

pub struct Income {
    pub amount: f64,
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50; // Переопределяем константу

   fn amount(&self) -> f64 {
       self.amount
   }

   fn set_amount(&mut self, new_amount: f64) {
       self.amount = new_amount;
   }
}

impl Taxable for Income {
    const TAX_RATE: f64 = 0.25;

    fn amount(&self) -> f64 {
        self.amount
    }

    fn set_amount(&mut self, new_amount: f64) {
       self.amount = new_amount;
   }
}