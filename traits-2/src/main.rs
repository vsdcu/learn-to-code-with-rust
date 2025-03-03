
trait Investment<T> {
    //getter method
    fn get_amount(&self) -> T;

    //setter
    fn set_amount(&mut self, amount: T);

    fn double_amount(&mut self);
}

trait Taxable: Investment<f64>{
    const TAX_RATE: f64 = 0.2;
    
    fn tax_bill(&self) -> f64 {
        self.get_amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64
}

#[derive(Debug)]
struct Bonus {
    value: f64
}

#[derive(Debug)]
struct QualityTime {
    hours: u32
}

impl Investment<f64> for Income {
    fn get_amount(&self) -> f64 {
        self.amount
    }
    
    fn set_amount(&mut self, new_value:f64) {
        self.amount = new_value;
    }

    fn double_amount(&mut self) {
        self.amount *= 2.0;
    }
}

impl Taxable for Income {

}


impl Investment<f64> for Bonus {
    fn get_amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_value: f64) {
        self.value = new_value;
    }

    fn double_amount(&mut self) {
        self.value *= 2.0
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.25;

}

impl Investment<u32> for QualityTime {
    fn get_amount(&self) -> u32 {
        self.hours
    }

    fn set_amount(&mut self, new_value: u32) {
        self.hours = new_value;
    }

    fn double_amount(&mut self) {
        self.hours *= 2;
    }
}


fn main() {

    let mut income = Income {amount: 200.0};
    income.double_amount();
    let tax_due = income.tax_bill();
    println!{"{tax_due} tax is due for your {:?}", income};


    //overriding associated constants example 
    let mut bonus = Bonus {value: 200.0};
    bonus.double_amount();
    let tax_due = bonus.tax_bill();
    println!{"{tax_due:.2} tax is due for your {:?}", bonus};

    let mut rust_programming_time = QualityTime { hours: 5000};
    rust_programming_time.double_amount();
    println!{"Rust prgramming time in hours {:?}", rust_programming_time};


}
