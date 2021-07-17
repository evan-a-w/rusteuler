use bit_vec::BitVec;
use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;
use std::collections::HashSet;
use std::cmp::*;

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

// assumes elemeents of v are distinct
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

pub fn prime_variants(n: usize) -> usize
{
    let mut max = 0;
    // for every digit we replace, loop over all combs and change, check if prime
    let dl = dig_list(n);
    let places: Vec<usize> = (0..dl.len()).collect();
    
    for i in 1..(dl.len()+1)
    {
        let cmbs = combs(&places, i);
        for cmbvec in &cmbs
        {
            let mut ct = 0;
            let mut ndl = dl.clone();

            for digit in 0..10
            {

                for &place in cmbvec
                {
                    ndl[place] = digit; 
                }
                if ndl[0] == 0
                {
                    continue;
                }
                let nn = list_dig(&ndl);
                if naive_prime_test(nn)
                {
                    ct += 1;
                }
            }
            
            if ct > max
            {
                max = ct;
            }
        }
    }
    
    max
}

pub fn prime_variants_debug(n: usize) -> usize
{
    let mut max = 0;
    // for every digit we replace, loop over all combs and change, check if prime
    let dl = dig_list(n);
    let places: Vec<usize> = (0..dl.len()).collect();
    
    for i in 1..(dl.len()+1)
    {
        let cmbs = combs(&places, i);
        for cmbvec in &cmbs
        {
            let mut ct = 0;
            let mut ndl = dl.clone();
            let mut hash: HashSet<usize> = HashSet::new();

            for digit in 0..10
            {

                for &place in cmbvec
                {
                    ndl[place] = digit; 
                }
                if ndl[0] == 0
                {
                    continue;
                }
                let nn = list_dig(&ndl);
                if naive_prime_test(nn)
                {
                    hash.insert(nn);
                    ct += 1;
                }
            }
            
            if ct > max
            {
                for i in hash
                {
                    print!("{} ", i);
                }
                print!(" | ");
                for &place in cmbvec
                {
                    print!("{} ", place);
                } 
                println!("");
                max = ct;
            }
        }
    }
    
    max
}

pub fn merge<T>(v: &mut Vec<T>, p: usize, q: usize, r: usize)
    where T: Ord + Copy
{
    if p > q || p >= r || q >= r
    {
        return;
    }

    let n1 = q - p + 1;
    let n2 = r -q;
    let mut L: Vec<T> = vec![v[0]; n1+1];
    let mut R: Vec<T> = vec![v[0]; n2+1];

    for i in 0..n1
    {
        L[i] = v[p+i];
    }
    for i in 0..n2
    {
        R[i] = v[q+i+1];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = p;
    while k <= r
    {
        if i < n1 && (L[i] <= R[j] || j >= n2)
        {
            v[k] = L[i];
            i += 1;
        }
        else
        {
            v[k] = R[j];
            j += 1;
        }

        k += 1;
    }
}

pub fn merge_sort<T>(v: &mut Vec<T>)
    where T: Ord + Copy
{
    go_merge(v, 0, v.len()-1); 
}

pub fn go_merge<T>(v: &mut Vec<T>, p: usize, q: usize)
    where T: Ord + Copy
{
    if q - p > 0
    {
        let m = (((p+q) as f64)/2.0).floor() as usize;
        go_merge(v, p, m);
        go_merge(v, m+1, q);
        merge(v, p, m, q);
    }
}

pub fn factorial(n: usize) -> BigUint
{
    let mut res = BigUint::from(n);
    for i in 2..n
    {
        res = res * i;
    }

    res
}

pub fn nchooser(n: usize, r: usize) -> BigUint
{
    let mut num: BigUint; 
    if r == 1
    {
        num = BigUint::from(n);
    }
    else
    {
        num = BigUint::from(n);
        for i in ((n-(r-1))..n).rev()
        {
            num = num * i;
        }
        let den = factorial(r);
        num = num / den;
    }

    num
}
