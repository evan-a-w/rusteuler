use crate::accs::*;
use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;

pub fn prob1() -> usize
{
    (3..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

pub fn prob2() -> usize
{
    let fib = fibs_while_less(4000000);
    fib.into_iter().filter(|x| x % 2 == 0).sum()
}

pub fn prob3() -> usize
{
    prime_factors_big(600851475143).into_iter().max().unwrap()
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

pub fn prob5() -> usize
{
    (1..21).fold(1, |l, x| lcm(l, x))
}

pub fn prob6() -> usize
{
    let s : i64 = (1..101).sum::<i64>();
    ((1..101).map(|x| x*x).sum::<i64>() - s*s).abs() as usize
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

pub fn prob51() -> usize
{
    let mut i: usize = 2;
    let mut max = 0;
    loop
    {
        if naive_prime_test(i)
        {
            if prime_variants(i) == 8
            {
                return i;
            }
        }
        i += 1;
    }
}

pub fn prob52() -> usize
{
    let mut i = 1;
    loop
    {
        let mut dl = dig_list(i);
        merge_sort(&mut dl);
        if (2..7).map(|x| {
                              let t = i*x;
                              let mut t = dig_list(t);
                              merge_sort(&mut t);
                              t
                          }).all(|x| x == dl)
        {
            break;
        }
        i += 1;
    }

    i
}

pub fn prob53() -> usize
{
    let mut count = 0;

    for n in 23..101
    {
        let mid = if n % 2 == 0 {n/2} else {n/2 + 1};
        
        let mut r = 1;
        let b = BigUint::from(1_000_000usize);
        while r < n
        {
            if nchooser(n, r) > b
            {
                // nCr > 1,000,000 so all nCk for k = r..(n-r) > 1,000,000
                // which means we need to increase count by (n-r)-r+1
                count += n - 2*r + 1;
                break;
            }
            r += 1;
        }
    }

    count
}
