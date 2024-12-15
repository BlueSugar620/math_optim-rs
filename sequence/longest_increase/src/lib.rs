pub fn lis<Value: Copy + Ord>(a: &[Value], strict: bool) -> (usize, Vec<usize>) {
    let mut dp = vec![];
    let mut idx = vec![];
    for a in a {
        let mut l = -1;
        let mut r = dp.len() as i64;
        while r - l > 1 {
            let mid = (l + r) / 2;
            let flag = if strict {
                dp[mid as usize] < *a
            } else {
                dp[mid as usize] <= *a
            };
            if flag {
                l = mid;
            } else {
                r = mid;
            }
        }
        let i = r as usize;
        if dp.len() <= i {
            dp.push(*a);
        } else {
            dp[i] = *a;
        }
        idx.push(i);
    }
    let mut lis = vec![];
    let mut pos = dp.len() - 1;
    for (i, &idx) in idx.iter().enumerate().rev() {
        if idx == pos {
            lis.push(i);
            if pos == 0 {
                break;
            } else {
                pos -= 1;
            }
        }
    }
    lis.reverse();
    (lis.len(), lis)
}
