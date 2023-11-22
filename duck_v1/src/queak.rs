pub trait QueakBehaviour {
    fn queak(&self);
}
pub struct Queak;
pub struct Sqeak;
pub struct MuteQueak;

impl QueakBehaviour for Queak {
    fn queak(&self) {
        println!("HEY ! i can do queak sound !!");
    }
}

impl QueakBehaviour for Sqeak {
    fn queak(&self) {
        println!("HEY ! i am rubber duck !!");
    }
}

impl QueakBehaviour for MuteQueak {
    fn queak(&self) {
        println!("HEY ! i can do silent queak !");
    }
}