mod atomics;
use atomics::atomics::atomics_example;
use atomics::fetch_and_modify::fetch_and_modify_examples;
use atomics::compare_and_exchange::compare_and_exchange_example;
fn main() {
    atomics_example();
    fetch_and_modify_examples();
    compare_and_exchange_example();
}
