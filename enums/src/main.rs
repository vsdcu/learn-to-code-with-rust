#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium{tier: Tier},
}

impl Subscription {
    fn summarize(&self){
        match self {
            Subscription::Free => {
                println!("You have limited access to the site.");
            }

            Subscription::Basic(price, months, ) => {
                println!("You have limited access to the site's premium features for {price} for {months} months");
            }

            Subscription::Premium { tier } => {
                println!("You have full access to the site's premium features. Your tier is {tier:?}");
            }
        }
    }
}


fn main() {

    let sub_free = Subscription::Free;
    let sub_basic = Subscription::Basic(20.0, 3);
    let sub_premium = Subscription::Premium { tier: Tier::Platinum };

    sub_free.summarize();
    sub_basic.summarize();
    sub_premium.summarize();


}
