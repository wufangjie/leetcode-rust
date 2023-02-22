impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            n
        } else {
            let n = n as usize;
            let mut cache = Vec::with_capacity(n + 1);
            cache.push(0);
            cache.push(1);
            for i in 2..=n as usize {
                cache.push(cache[i - 2] + cache[i - 1])
            }
            cache[n]
        }
    }
}
