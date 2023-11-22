use duck_v1::mallardduck::MallardDuck;
use duck_v1::fly::FlyNoWay;
use duck_v1::queak::MuteQueak;
fn main () {
    let mut duck = MallardDuck::new();
    duck.duck.perform_fly();
    duck.duck.perform_queak();
    duck.duck.set_fly_behaviour(Box::new(FlyNoWay));
    duck.duck.set_queak_behaviour(Box::new(MuteQueak));
    duck.duck.perform_fly();
    duck.duck.perform_queak();
}