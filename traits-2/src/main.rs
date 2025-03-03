
trait Investment {
    //getter method
    fn get_amount(&self) -> f64;

    //setter
    fn set_amount(&mut self, amount: f64);

    //utilizing setter and getter both to gain access to state data of the object
    fn double_amount(&mut self) {
        self.set_amount(self.get_amount() * 2.0); 
    }
}

trait Taxable: Investment{
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
    hours: f64
}

impl Investment for Income {
    fn get_amount(&self) -> f64 {
        self.amount
    }
    
    fn set_amount(&mut self, new_value: f64) {
        self.amount = new_value;
    }
}

impl Taxable for Income {

}


impl Investment for Bonus {
    fn get_amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_value: f64) {
        self.value = new_value;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.25;

}

impl Investment for QualityTime {
    fn get_amount(&self) -> f64 {
        self.hours
    }

    fn set_amount(&mut self, new_value: f64) {
        self.hours = new_value;
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

    let mut weekend = QualityTime { hours: 5.0};
    weekend.double_amount();
    println!{"Weekend fun time in hours {:?}", weekend};


}
