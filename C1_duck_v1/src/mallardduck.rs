use crate::duck::Duck;
use crate::fly::FlyWithWings;
use crate::queak::Queak;
pub struct MallardDuck {
    pub duck : Duck
    // we can add more functionalities if needed here 
}

impl MallardDuck {
    pub fn new() -> Self {
        MallardDuck {
            duck : Duck {
                fly_behaviour : Box::new(FlyWithWings),
                queak_beahviour : Box::new(Queak)
            }
        }
    }

    // also we can add more functions depending on new features
}