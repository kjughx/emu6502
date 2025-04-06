pub mod hardware;
pub mod types;

#[macro_export]
macro_rules! mutex(
    ($b:expr) => (
        Arc::new(Mutex::new($b))
    )
);

#[macro_export]
macro_rules! dbg_bin(
    ($b:expr) => (
        println!("{}: {:#09b}", file!(), $b)
    );
);

#[macro_export]
macro_rules! dbg_byte(
    ($b:expr) => (
        println!("{}: {:#04X}", file!(), $b)
    );
);

#[macro_export]
macro_rules! dbg_addr(
    ($b:expr) => (
        println!("{}: {:#06X}", file!(), $b)
    )
);
