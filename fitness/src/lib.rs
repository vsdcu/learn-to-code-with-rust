/// library crate

pub mod cardio;
pub mod weightlifting;
mod diet {
    const NUTRITIONIST: &str = "Norah Nutrition";
    pub fn ask_about_program() {
        println!("The nutritionist is {}", NUTRITIONIST);
    }
}

use cardio::Exercise as CardioExercise;
use weightlifting::Exercise as WeightliftingExercise;


#[derive(Debug)]
pub struct GymWorkout {
    cardio: CardioExercise,
    weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    pub fn new(cardio: CardioExercise, weightlifting: WeightliftingExercise) -> Self {
        
        cardio::ask_about_program();
        weightlifting::ask_about_program();
        diet::ask_about_program();
        
        Self { cardio, weightlifting }
    }
}