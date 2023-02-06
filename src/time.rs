use std::time::UNIX_EPOCH;

pub type Seconds = u32;

pub fn since_epoch() -> Seconds {
    UNIX_EPOCH.elapsed().unwrap().as_secs() as u32 // ok to unwrap as
                                                   // UNIX_EPOCH happened a long time ago
}
