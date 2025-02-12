// min-version: 1.30.0

// === LLDB TESTS ==================================================================================

// lldb-command:run

// lldb-command:print r
// lldbr-check:[...]r = strong=2, weak=1 { value = 42 }
// lldbg-check:[...]$0 = strong=2, weak=1 { value = 42 }
// lldb-command:print a
// lldbr-check:[...]a = strong=2, weak=1 { data = 42 }
// lldbg-check:[...]$1 = strong=2, weak=1 { data = 42 }

// === GDB TESTS ==================================================================================

// gdb-command:run

// gdb-command:print r
// gdb-check:[...]$1 = strong=2, weak=1 = {value = 42, strong = 2, weak = 1}
// gdb-command:print a
// gdb-check:[...]$2 = strong=2, weak=1 = {value = 42, strong = 2, weak = 1}

use std::rc::Rc;
use std::sync::Arc;

fn main() {
    let r = Rc::new(42);
    let r1 = Rc::clone(&r);
    let w1 = Rc::downgrade(&r);

    let a = Arc::new(42);
    let a1 = Arc::clone(&a);
    let w2 = Arc::downgrade(&a);

    print!(""); // #break
}
