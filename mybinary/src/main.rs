// 1) Uncomment this
// 2) Comment line 7 in `mycriticalimpl/lib.rs`
// ... and linking will magically succeed!
// critical_section::set_impl!(mycriticalimpl::cs::MyCriticalSection);

fn main() {
    println!("Calling magic: {}", mycriticalimpl::magic::magic());

    println!(
        "Computation from critical section: {}",
        mylib::in_critical_section()
    );
}
