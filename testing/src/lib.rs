/* src/lib.rs */
#[derive(Debug, PartialEq)]
pub struct Athlete {
    pub points_scored: i32,
}

impl Athlete {
    pub fn scored_more_than(&self, other_athlete: &Athlete) -> bool {
        self.points_scored > other_athlete.points_scored
    }

    pub fn subtract_points(&mut self, amount: i32) -> Result<(), String> {
        if amount > self.points_scored {
            Err(String::from("Athlete cannot drop below 0 points"))
        } else {
            self.points_scored -= amount;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn athlete_scored_more_than() {
        let winner = Athlete { points_scored: 100 };
        let loser = Athlete { points_scored: 50 };

        assert!(
            winner.scored_more_than(&loser),
            "The scored_more_than method should have returned true. The winner {winner:#?} should have scored more than the {loser:#?} Athlete."
        );
    }

    #[test]
    fn athletes_with_same_points_are_equal() {
        let one = Athlete { points_scored: 25 };
        let two = Athlete { points_scored: 25 };

        assert_eq!(one, two);
    }

    #[test]
    fn subtracts_points_from_impolite_athlete() {
        let mut athlete = Athlete { points_scored: 10 };
        let result = athlete.subtract_points(3);

        assert!(result.is_ok());
        assert_eq!(athlete.points_scored, 7);
    }

    #[test]
    fn cannot_subtract_more_points_than_athlete_has() -> Result<(), String> {
        let mut athlete = Athlete { points_scored: 10 };
        let result = athlete.subtract_points(90);

        match result {
            Ok(_) => Err(String::from(
                "Subtracted more points from Athlete than should have been possible",
            )),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn success() {
        println!("From the success test");
        assert!(true);
    }
}
