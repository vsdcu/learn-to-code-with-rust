use fitness::GymWorkout;
use fitness::cardio::{CardioTool, Exercise as CardioExercise};
use fitness::weightlifting::Exercise as WeightliftingExercise;

fn main() {
    println!("Fitness..");

    let cardio_ex= CardioExercise::new(String::from("Sunday"), CardioTool::Bike, 30);
    let weight_ex= WeightliftingExercise::new(String::from("Dead Lift"), 10);

    println!("{:#?}", cardio_ex);
    println!("{:#?}", weight_ex);
    
    let workout = GymWorkout::new(cardio_ex, weight_ex);
    println!("{:#?}", workout);

}
