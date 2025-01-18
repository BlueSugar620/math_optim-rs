pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub fn extgcd(a: u64, b: u64) -> (i64, i64, u64) {
    if b == 0 {
        return (1, 0, a);
    }
    let (x, y, g) = extgcd(b, a % b);
    (y, x - (a / b) as i64 * y, g)
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub fn factorize(mut n: u64) -> Vec<(u64, usize)> {
    let mut res = Vec::new();
    for k in 2.. {
        if k * k > n {
            break;
        }
        if n % k > 0 {
            continue;
        }
        let mut c = 0;
        while n % k == 0 {
            n /= k;
            c += 1;
        }
        res.push((k, c));
    }
    if n > 1 {
        res.push((n, 1));
    }
    res
}

pub fn divisors(n: u64) -> Vec<u64> {
    let f = factorize(n);
    let mut res = f.iter().fold(vec![1], |s, &(p, a)| {
        let mut res = Vec::with_capacity(s.len() * (a + 1));
        res.extend_from_slice(&s);
        let mut pow = 1;
        for _ in 1..=a {
            pow *= p;
            for s in s.iter() {
                res.push(pow * s);
            }
        }
        res
    });
    res.sort();
    res
}
