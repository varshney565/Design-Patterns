use crate::{fly::FlyBehaviour, queak::QueakBehaviour};

pub struct Duck {
    pub fly_behaviour : Box<dyn FlyBehaviour>,
    pub queak_beahviour : Box<dyn QueakBehaviour>
}

impl Duck {
    pub fn perform_fly(&self) {
        self.fly_behaviour.fly();
    }

    pub fn perform_queak(&self) {
        self.queak_beahviour.queak();
    }

    pub fn swim(&self) {
        println!("HEY ! i can swim !! ");
    }

    pub fn display(&self) {
        println!("Hey ! i am duck !!");
    }

    pub fn set_fly_behaviour(&mut self,fb : Box<dyn FlyBehaviour>) {
        self.fly_behaviour = fb;
    }

    pub fn set_queak_behaviour(&mut self,qb : Box<dyn QueakBehaviour>) {
        self.queak_beahviour = qb;
    }
}
