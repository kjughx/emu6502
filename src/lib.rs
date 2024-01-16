pub mod hardware;
pub mod instruction;
pub mod types;


#[macro_export]
macro_rules! Mutex(
    ($b:expr) => (
        Arc::new(Mutex::new($b))
    )
);

#[macro_export]
macro_rules! dbg_byte(
    ($b:expr) => (
        println!("{:#04X}", $b)
    )
);

#[macro_export]
macro_rules! dbg_addr(
    ($a:expr) => (
        println!("{:#06X}", $a)
    )
);
