type N = Vec<u8>;

fn pow2(n: &mut N) {
    let mut c = 0;
    for x in n.iter_mut() {
        *x = *x * 2 + c;
        if *x >= 10 {
            *x -= 10;
            c = 1;
        } else {
            c = 0;
        }
    }
    if c == 1 {
        n.push(1);
    }
}

#[test]
fn test_pow2() {
    let mut n = vec![1];
    pow2(&mut n);
    assert_eq!(n, vec![2]);
    for _ in 0..10 {
        pow2(&mut n);
    }
    assert_eq!(n, vec![8, 4, 0, 2]);
}

fn count_digit(n: &N) -> [u32; 10] {
    let mut count = [0; 10];
    for &x in n.iter() {
        count[x as usize] += 1;
    }
    count
}

fn main() {
    let mut n = vec![1];
    let mut e = 0u64;
    loop {
        let digit = count_digit(&n);
        let c = *digit.iter().min().unwrap();
        println!("{} {}", e, c);
        pow2(&mut n);
        e += 1;
    }
}
