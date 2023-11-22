# Duck Simulator

This Rust project implements a simple duck simulator that allows for flexible behavior composition for ducks. Ducks can have different flying and quacking behaviors, and the simulator demonstrates how these behaviors can be changed at runtime.

## Project Structure

### `lib.rs`

- `fly`: Module containing traits and implementations for flying behaviors.
- `queak`: Module containing traits and implementations for quacking behaviors.
- `duck`: Module defining the `Duck` struct and its methods for performing actions like flying, quacking, swimming, and displaying.
- `mallardduck`: Module implementing a specific type of duck, the `MallardDuck`, with its own set of default behaviors.

### `duck.rs`

Defines the `Duck` struct with methods for performing various actions and modifying behaviors at runtime.

### `fly.rs`

Contains the `FlyBehaviour` trait and implementations for different flying behaviors, such as `FlyWithWings` and `FlyNoWay`.

### `main.rs`

Example usage of the duck simulator, creating a `MallardDuck` and demonstrating how to perform actions with different behaviors.

### `mallardduck.rs`

Implements the `MallardDuck` struct, a specific type of duck with default flying and quacking behaviors.

### `queak.rs`

Contains the `QueakBehaviour` trait and implementations for different quacking behaviors, such as `Queak`, `Sqeak`, and `MuteQueak`.

## Example Usage

```rust
use duck_v1::mallardduck::MallardDuck;
use duck_v1::fly::FlyNoWay;
use duck_v1::queak::MuteQueak;

fn main() {
    // Create a MallardDuck with default behaviors
    let mut duck = MallardDuck::new();

    // Perform default behaviors
    duck.duck.perform_fly();
    duck.duck.perform_queak();

    // Change behaviors at runtime
    duck.duck.set_fly_behaviour(Box::new(FlyNoWay));
    duck.duck.set_queak_behaviour(Box::new(MuteQueak));

    // Perform modified behaviors
    duck.duck.perform_fly();
    duck.duck.perform_queak();
}
```