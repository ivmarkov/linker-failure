// Pub so that we can call `critical_section::set_impl!(MyCriticalSection);` from the binary crate
use critical_section::RawRestoreState;

pub struct MyCriticalSection;
critical_section::set_impl!(MyCriticalSection);

unsafe impl critical_section::Impl for MyCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        // TODO
    }

    unsafe fn release(_token: RawRestoreState) {
        // TODO
    }
}
