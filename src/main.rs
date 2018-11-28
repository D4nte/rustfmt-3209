#[derive(Debug)]
pub struct KeyStore {
    _master_privkey: String,
    transient_root_privkey: String,
    internal_root_privkey: String,
    // TODO: replace with AtomicU32 once stable https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU32.html
    next_internal_index: Mutex<u32>,
    // Do we want to remember already generated addresses or regenerate them?
    // Memory vs CPU -> could be a switch/option
    // Common practice for wallets is to pre-generate some addresses, hence:
    // TODO: manage a key pool
    // - key ready for use (pool)
    // - key already used
}

fn main() {
    println!("Hello, world!");
}
