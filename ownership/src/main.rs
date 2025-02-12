fn main() {
    let is_concert = true;
    let is_event = is_concert;
    
    println!("No ownership move, my variables {is_concert} and {is_event}");
    
    let sushi = "Salmon";
    let dinner = sushi;
    
    println!("No ownership move, my variables {sushi} and {dinner}");
    
    // heap
    let sushi = String::from("Salmon");
    let dinner = sushi;
    
    println!("Yes ownership move (sushi no more owner now), my variables {dinner}");
    
    let result = eat_meal(dinner); // ownership moved
    println!("The return value {result}");
    
    fn eat_meal(mut meal: String) -> String {
        meal.clear(); // this will clean the string value. i.e Salmon 
        return meal;
    }
    
    //-------- challange 2
    
    
    let mut trip = start_trip();
    
    visit_philadelphia(&mut trip);
    trip.push_str(" and ");
    visit_new_york(&mut trip);
    trip.push_str(" and ");
    visit_boston(&mut trip);
    trip.push('.');
    
    show_itinerary(&trip);
    
    // copy trait NOT implemeted by the String, hence rust need reference of value
    let array = [String::from("abc"), String::from("xyz")];
    let first_element = &array[0];
    println!("{first_element}, {array:?}");
    
    
    // copy trait implemeted by the int, hence rust will copy the data
    let array_int = [1,5];
    let first_element = array_int[0];
    println!("{first_element}, {array_int:?}");
    
    // copy trait implemeted by the &str litrals, hence rust will copy the data
    let array_str = ["str-1","str-2"];
    let first_element = array_str[0];
    println!("{first_element}, {array_str:?}");
    
    }
    
    
    fn start_trip() -> String{
     let initial_plan = String::from("The plan is...");
     return initial_plan;
    }
    
    fn visit_philadelphia(trip: &mut String) {
        trip.push_str("Philadephia");
    }
    
    fn visit_new_york(trip: &mut String) {
        trip.push_str("New York");
    }
    
    fn visit_boston(trip: &mut String) {
        trip.push_str("Boston");
    }
    
    fn show_itinerary(trip: &String) {
        println!("{trip}");
    }