use bit_vec::BitVec;
use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;
use std::collections::HashSet;

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

pub fn naive_prime_test(n : usize) -> bool
{
    (2..((n as f64).sqrt().ceil() as usize) + 1).all(|x| n % x != 0)
}

pub fn simple_sieve(n : usize) -> BitVec
{
    // From pseudocode on wikipedia
    // Indexed 2 to n => n-1 values (indexed 0 to n-2)
    //let mut res : Vec<bool> = vec![true; n-1];
    let mut res : BitVec = BitVec::from_elem(n-1, true);
    for i in 2..((n as f64).sqrt().ceil() as usize)
    {
        if res[i-2]
        {
            let mut j : usize = i*i;
            while j <= n
            {
                res.set(j-2, false);
                j += i;
            }
        }
    }

    //(2..(n+1)).filter(|x| res[x-2]).collect::<Vec<usize>>()
    res
}

pub fn prime_factors_small(n : usize) -> Vec<usize>
{
    let mut res : Vec<usize> = Vec::new();
    let primes = simple_sieve(n);

    for i in 2..n
    {
        if primes[i-2] && n % i == 0
        {
            res.push(i);
        }
    }

    if res.len() == 0
    {
        res.push(n);
    }

    res
}

pub fn prime_factors_big(mut n : usize) -> Vec<usize>
{
    let mut res : Vec<usize> = Vec::new();
    let mut i = 2;
    
    while n > 1
    {
        if naive_prime_test(i) && n % i == 0
        {
            res.push(i);
            n /= i;
        }

        i += 1;
    }

    res
}

pub fn prob3() -> usize
{
    prime_factors_big(600851475143).into_iter().max().unwrap()
}

pub fn dig_list(mut n : usize) -> Vec<u8>
{
    let mut res : Vec<u8> = Vec::new();
    while n > 0
    {
        let curr : u8 = (n % 10) as u8;
        res.push(curr);
        n /= 10;
    }

    res.reverse();
    res
}

pub fn list_dig(vec : &Vec<u8>) -> usize
{
    let mut res : usize = 0;
    for i in 0..vec.len()
    {
        res *= 10;
        res += vec[i] as usize;
    }

    res
}

pub fn is_palindrome<T: std::cmp::PartialEq>(vec : &Vec<T>) -> bool
{
    let mut h = vec.len()-1;
    let mut l = 0;
    while h > l
    {
        if vec[l] != vec[h]
        {
            return false;
        }
        h -= 1;
        l += 1;
    }

    true
}

pub fn prob4() -> usize
{
    let mut max : usize = 0;

    for i in 100..1000
    {
        for j in 100..1000
        {
            if is_palindrome(&dig_list(j*i)) && i*j > max
            {
                max = i*j;
            }
        }
    }

    max
}

pub fn lcm(x : usize, y : usize) -> usize
{
    let mut res : usize;
    let mut l : usize;
    if x > y
    {
        res = x;
        l = y;
    }
    else
    {
        res = y;
        l = x;
    }
    let h = res;

    while res % l != 0
    {
        res += h;
    }

    res
}

pub fn prob5() -> usize
{
    (1..21).fold(1, |l, x| lcm(l, x))
}

pub fn prob6() -> usize
{
    let s : i64 = (1..101).sum::<i64>();
    ((1..101).map(|x| x*x).sum::<i64>() - s*s).abs() as usize
}

pub fn segmented_sieve(n : usize) -> Vec<usize>
{
    let mut del = (n as f64).sqrt().floor() as usize;
    let mut l : usize = del + 1;
    let mut h : usize = 2*del; // go from del + 1 to 2*del
    let bv = simple_sieve(del);
    let mut primes : Vec<usize> = Vec::new();

    for i in 2..del
    {
        if bv[i-2]
        {
            primes.push(i);
        }
    }

    let mut first_over : bool = true;
    while h <= n
    {
        let mut nbv = BitVec::from_elem(del, true);

        let max = (h as f64).sqrt().floor() as usize;
        for &i in &primes
        {
            if i > max { break; }
            let mut index_n = (((l as f64) / (i as f64)).ceil() as usize)*i - l;
            while index_n < del
            {
                nbv.set(index_n, false);
                index_n += i;
            }
        }

        for i in 0..del
        {
            if nbv[i] { primes.push(i + l); }
        }

        l = h + 1;
        h += del;

        if h > n && first_over
        {
            let dif = h - n;
            del -= dif;
            h = n;
            first_over = false;
        }
    }

    primes
}

// 0.03 user in release mode
pub fn prob7_naive() -> usize
{
    let mut i = 2;
    let mut count = 0;
    loop
    {
        if naive_prime_test(i)
        {
            count += 1;
            if count == 10000
            {
                return i;
            }
        }
        i += 1;
    }
}

// 0.01 user
pub fn prob7_sieve() -> usize
{
    segmented_sieve(110000)[9999]    
}

pub fn prob8() -> usize
{
    let s : String = String::from("73167176531330624919225119674426574742355349194934\
                                  96983520312774506326239578318016984801869478851843\
                                  85861560789112949495459501737958331952853208805511\
                                  12540698747158523863050715693290963295227443043557\
                                  66896648950445244523161731856403098711121722383113\
                                  62229893423380308135336276614282806444486645238749\
                                  30358907296290491560440772390713810515859307960866\
                                  70172427121883998797908792274921901699720888093776\
                                  65727333001053367881220235421809751254540594752243\
                                  52584907711670556013604839586446706324415722155397\
                                  53697817977846174064955149290862569321978468622482\
                                  83972241375657056057490261407972968652414535100474\
                                  82166370484403199890008895243450658541227588666881\
                                  16427171479924442928230863465674813919123162824586\
                                  17866458359124566529476545682848912883142607690042\
                                  24219022671055626321111109370544217506941658960408\
                                  07198403850962455444362981230987879927244284909188\
                                  84580156166097919133875499200524063689912560717606\
                                  05886116467109405077541002256983155200055935729725\
                                  71636269561882670428252483600823257530420752963450");

    let mut max : usize = 0;

    for i in 0..(s.len()-14)
    {
        let cv: usize = s[i..i+13].chars()
                                  .map(|x| x.to_digit(10).unwrap())
                                  .fold(1, |prod, x| prod*(x as usize));
        if cv > max { max = cv; }
    }

    max
}

pub fn prob9() -> usize
{
    for i in 1..1000
    {
        for j in 1..1000
        {
            let c = ((i*i + j*j) as f64).sqrt();
            if c.fract() == 0.0 && i + j + (c as usize) == 1000
            {
                return i*j*(c as usize);
            }
        }
    }

    0
}

pub fn prob10() -> BigUint
{
    let mut res : BigUint = Zero::zero();
    let primes = segmented_sieve(2000000); 
    for i in primes
    {
        res += i.to_biguint().unwrap();
    }

    res
}

pub fn dig_listu32(mut n : usize) -> Vec<u32>
{
    let mut res : Vec<u32> = Vec::new();
    while n > 0
    {
        let curr : u32 = (n % 10) as u32;
        res.push(curr);
        n /= 10;
    }

    res
}

pub fn factors(n : usize) -> Vec<usize>
{
    let mut res : Vec<usize> = vec![1];

    for i in 2..((n as f64).sqrt() as usize)
    {
        if n % i == 0
        {
            res.push(i);
            res.push(n / i);
        }
    }

    res.push(n);
    res
}

pub fn prob12() -> usize
{
    let mut i = 1;
    loop
    {
        let n : usize = (i*(i+1))/2; 
        let fs = factors(n);

        if fs.len() > 500
        {
            return n;
        }

        i += 1;
    }
}

pub fn prob53() -> usize
{
    let mut i: usize = 1;
    loop
    {
        let mut dl = dig_list(i); 
    }
}

pub fn combs<T: Copy + Eq>(v: &Vec<T>, n: usize) -> Vec<Vec<T>>
{
    let mut res: Vec<Vec<T>> = Vec::new();
    if n == 1
    {
        for &i in v
        {
            res.push(vec![i]);
        }
    }
    else if v.len() == n
    {
        res.push(v.clone());
    }
    else if v.len() > n
    {
        for i in 0..v.len()
        {
            let mut tv: Vec<T> = Vec::new();
            for j in (i+1)..v.len()
            {
                tv.push(v[j]);
            }
            let nr: Vec<Vec<T>> = combs(&tv, n-1);
            for mut x in nr
            {
                x.push(v[i]);
                res.push(x);
            }
        }
    }

    res
}
