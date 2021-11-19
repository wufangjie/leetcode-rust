pub mod utils;

// https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
// NOTE: must be used for a debug purpose only:
pub fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

// `#[macro_export]` will be exported at the root of the crate
// NOTE: dbg!(var1, var2) is ok, but dbgt! cannot
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
                    $crate::type_of(tmp), // not $val, &tmp
                    &tmp
                );
                tmp
            }
        }
    };
}

// seems no need to support statement,
// +put block before expr, block belongs to expr?+
// recursive macro is ok
// usage: timeit!(n, code) or timeit!(code) (=== timeit!(1, code))
// code can be blocks, or just simple expressions
#[macro_export]
macro_rules! timeit {
    ($loops:expr, $code:expr) => {
        let timeit_n = $loops;
        let timeit_start = std::time::Instant::now();
        for _ in 0..timeit_n {
            $code;
        }
        let timeit_cost = timeit_start.elapsed();
        println!(
            "[{}:{}] ({} loops, {:?} per loop)\t{{ {} }}",
            file!(),
            line!(),
            timeit_n,
            timeit_cost / timeit_n,
            match stringify!($code) {
                s if s.contains(";") => "...",
                s => s,
            }
        );
    };
    ($code:expr) => {
        timeit!(1, $code);
    };
}
