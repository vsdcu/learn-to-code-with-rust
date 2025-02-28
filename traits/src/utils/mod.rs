use crate::lodging::{Description, Accommodation};

// example of trait bounds
pub fn book_for_one_night<T: Accommodation>(entity: &mut T, guest_name: &str) {
    entity.book(guest_name, 1);
}

//mix n match, when two different objects needs to be passed
pub fn mix_and_match(first: &mut (impl Accommodation + Description), 
                 second: &mut impl Accommodation, 
                 guest_name: &str) {
    first.book(guest_name, 2);
    first.get_description();
    second.book(guest_name, 3);
}

//using where clause with generics
//mix n match, when two different objects needs to be passed
pub fn mix_and_match_generics<T, U>(first: &mut T, 
                 second: &mut U, 
                 guest_name: &str) 
                 where 
                    T: Accommodation + Description, 
                    U: Accommodation {
    first.book(guest_name, 2);
    first.get_description();
    second.book(guest_name, 3);
}