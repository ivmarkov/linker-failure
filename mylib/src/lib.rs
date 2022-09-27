pub fn in_critical_section() -> usize {
    critical_section::with(|_cs| 42)
}
