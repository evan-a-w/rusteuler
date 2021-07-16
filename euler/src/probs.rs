use math::round;
use bitvec::prelude::*;

pub fn prob1() -> usize
{
    (3..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

pub fn fibs(n : usize) -> Vec<usize>
{
    if n < 3
    {
        return vec![1; n];
    }
    let mut res : Vec<usize> = vec![1; n];
    for i in 2..n
    {
        res[i] = res[i-1] + res[i-2];
    }
    
    res
}

pub fn fibs_while_less(n : usize) -> Vec<usize>
{
    let mut res : Vec<usize> = vec![1, 1];
    let mut cur : usize = res[0] + res[1];
    
    let mut i : usize = 3;
    while cur < n
    {
        res.push(cur);
        cur = res[i-1] + res[i-2];
        i += 1;
    }

    res
}

pub fn prob2() -> usize
{
    let fib = fibs_while_less(4000000);
    fib.into_iter().filter(|x| x % 2 == 0).sum()
}

pub fn naiive_prime_test(n : usize) -> bool
{
    (2..((n as f64).sqrt().ceil() as usize) + 1).all(|x| n % x != 0)
}

pub fn prime_factors(n : usize) -> Vec<usize>
{
    let mut res : Vec<usize> = Vec::new();
    let mut s : usize = 2;
    loop
    {
        for i in s..((n as f64).sqrt().ceil() as usize)
        {
            if n % i == 0
            {
                res.push(i);
                n /= i;
                s = i + 1;
                ex = true;
            }
            if ex
            {
                break;
            }
        }
    }

    res
}
