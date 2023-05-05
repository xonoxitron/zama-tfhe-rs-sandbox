#[path = "circuits/boolean_circuit.rs"]
mod boolean_circuit;
#[path = "circuits/integer_circuit.rs"]
mod integer_circuit;
#[path = "circuits/shortint_circuit.rs"]
mod shortint_circuit;

fn main() {
    boolean_circuit::run();
    integer_circuit::run();
    shortint_circuit::run();
}
