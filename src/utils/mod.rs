pub mod heap;
pub use heap::Heap;

pub mod queue;
pub use queue::Queue;

pub mod stack;
pub use stack::Stack;

/* fn product(n: u16, k: u8) -> Vec<u16> {
    let mut ret = vec![];
    if k == 0 {
        ret.push(0);
    } else {
        let poss = product(n, k - 1);
        for comb in poss {
            let comb10 = comb * 10;
            for v in 0..n {
                ret.push(comb10 + v);
            }
        }
    }
    ret
} */
//println!("{:#?}", product(3, 3));

pub mod timer;
pub use timer::Timer;

//    = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
// help: a macro with this name exists at the root of the crate
//    |
// 36 | pub use ::dbgt;
//    |         ^^^^^^

// https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
// NOTE: must be used for a debug purpose only:
pub fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

// NOTE: dbg!(var1, var2) is ok, but dbgt! not
// NOTE: do not use expression(with side effects, for example borrow_mut()) in dbgt!
#[macro_export]
macro_rules! dbgt {
    ($val:expr) => {
        match $val {
            tmp => {
                eprintln!(
                    "[{}:{}] ({}: {}) = {:#?}",
                    file!(),
                    line!(),
                    stringify!($val),
                    leetcode_rust::utils::type_of(tmp), // $val, &tmp
                    &tmp
                );
                tmp
            }
        }
    };
}
