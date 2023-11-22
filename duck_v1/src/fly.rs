pub trait FlyBehaviour {
    fn fly(&self);
}
pub struct FlyNoWay;
pub struct FlyWithWings;

impl FlyBehaviour for FlyWithWings{
    fn fly(&self) {
        println!("HEY ! i can FLY !!");
    }
}

impl FlyBehaviour for FlyNoWay {
    fn fly(&self) {
        println!("HEY ! i can't FLY !!");
    }
}