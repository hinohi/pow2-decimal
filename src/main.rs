type N = Vec<u8>;

fn pow2(n: &mut N) {
    let mut c = 0;
    for x in n.iter_mut() {
        *x = *x * 2 + c;
        if *x >= 100 {
            *x -= 100;
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
    assert_eq!(n, vec![48, 20]);
}

fn count_digit(n: &N) -> [u32; 10] {
    let mut count = [0; 10];
    for &x in n.iter() {
        if x >= 10 {
            count[(x % 10) as usize] += 1;
            count[(x / 10) as usize] += 1;
        } else {
            count[x as usize] += 1;
        }
    }
    count
}

fn main() {
    let mut n = vec![1];
    for e in 0..1_000_000 {
        let digit = count_digit(&n);
        let c = *digit.iter().min().unwrap();
        println!("{} {}", e, c);
        pow2(&mut n);
    }
}
