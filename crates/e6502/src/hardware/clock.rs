use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Condvar, Mutex};

/// Each CPU machine instruction takes up a certain number of clock
/// cycles, usually equal to the number of memory accesses. For
/// example, the absolute indexing mode of the ORA instruction takes 4
/// clock cycles; 3 cycles to read the instruction and 1 cycle to read
/// the value of the absolute address. If no memory is accessed, the
/// number of clock cycles is two. The minimum clock cycles for any
/// instruction is two. When using indexed addressing, if the result
/// crosses a page boundary an extra clock cycle is added. Also, when
/// a zero page address is used in indexing mode (e.g. zp,X) an extra
/// clock cycle is added.
pub struct Clock {
    ticks: AtomicU64,
    state: Mutex<bool>,
    cvar: Condvar,
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            ticks: AtomicU64::new(0),
            state: Mutex::new(false),
            cvar: Condvar::new(),
        }
    }
    pub fn ticks(&self) -> u64 {
        self.ticks.load(Ordering::Relaxed)
    }

    pub fn tick(&self) {
        let mut state = self.state.lock().unwrap();
        while *state {
            state = self.cvar.wait(state).unwrap();
        }

        *state = !*state;
        self.cvar.notify_all();
    }

    pub fn tock(&self) {
        let mut state = self.state.lock().unwrap();
        while !*state {
            state = self.cvar.wait(state).unwrap();
        }

        *state = !*state;
        self.ticks.fetch_add(1, Ordering::Relaxed);
        self.cvar.notify_all();
    }

    pub fn wait_tick(&self) {
        let mut state = self.state.lock().unwrap();
        while !*state {
            state = self.cvar.wait(state).unwrap();
        }
    }

    pub fn wait_tock(&self) {
        let mut state = self.state.lock().unwrap();
        while *state {
            state = self.cvar.wait(state).unwrap();
        }
    }
}
