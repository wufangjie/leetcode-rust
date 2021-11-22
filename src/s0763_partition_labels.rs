struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut seq = [99usize; 26];
        let mut first = [0; 26];
        let mut last = [0; 26];
        let mut cur = 0usize;
        let mut i = 0;
        for c in s.chars() {
            let c_usize = c as u8 as usize - 97;
            if seq[c_usize] == 99 {
                seq[c_usize] = cur;
                first[cur] = i;
                last[cur] = i;
                cur += 1;
            } else {
                last[seq[c_usize]] = i;
            }
            i += 1;
        }

        let mut ret = vec![];
        let mut lo = 0;
        let mut hi = 0;
        for i in 0..cur {
            if first[i] > hi {
                ret.push(hi - lo + 1);
                lo = first[i];
                hi = last[i];
            } else if last[i] > hi {
                hi = last[i];
            }
        }
        ret.push(hi - lo + 1);
        ret
    }
}

#[test]
fn test_763() {
    assert_eq!(
        vec![10],
        Solution::partition_labels("eccbbbbdec".to_owned())
    );
    assert_eq!(
        vec![9, 7, 8],
        Solution::partition_labels("ababcbacadefegdehijhklij".to_owned())
    );
    assert_eq!(
        vec![9, 1],
        Solution::partition_labels("eaaaabaaec".to_owned())
    );
}
