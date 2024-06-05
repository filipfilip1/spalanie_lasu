mod forest;
mod simulation;
mod tree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simulation::run_simulation()
}
