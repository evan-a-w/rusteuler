use bit_vec::BitVec;
use num_bigint::BigUint;
use palindrome;
use std::cmp::*;
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;
use unicode_segmentation::UnicodeSegmentation;
use rustlib::primes::*;

pub fn fibs(n: usize) -> Vec<usize> {
    if n < 3 {
        return vec![1; n];
    }
    let mut res: Vec<usize> = vec![1; n];
    for i in 2..n {
        res[i] = res[i - 1] + res[i - 2];
    }

    res
}

pub fn fibs_while_less(n: usize) -> Vec<usize> {
    let mut res: Vec<usize> = vec![1, 1];
    let mut cur: usize = res[0] + res[1];

    let mut i: usize = 3;
    while cur < n {
        res.push(cur);
        cur = res[i - 1] + res[i - 2];
        i += 1;
    }

    res
}

pub fn naive_prime_test(n: usize) -> bool {
    (2..((n as f64).sqrt().ceil() as usize) + 1).all(|x| n % x != 0)
}

pub fn simple_sieve(n: usize) -> BitVec {
    // From pseudocode on wikipedia
    // Indexed 2 to n => n-1 values (indexed 0 to n-2)
    //let mut res : Vec<bool> = vec![true; n-1];
    let mut res: BitVec = BitVec::from_elem(n - 1, true);
    for i in 2..((n as f64).sqrt().ceil() as usize) {
        if res[i - 2] {
            let mut j: usize = i * i;
            while j <= n {
                res.set(j - 2, false);
                j += i;
            }
        }
    }

    //(2..(n+1)).filter(|x| res[x-2]).collect::<Vec<usize>>()
    res
}

pub fn prime_factors_small(n: usize) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    let primes = simple_sieve(n);

    for i in 2..n {
        if primes[i - 2] && n % i == 0 {
            res.push(i);
        }
    }

    if res.len() == 0 {
        res.push(n);
    }

    res
}

pub fn prime_factors_big(mut n: usize) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    let mut i = 2;

    while n > 1 {
        if naive_prime_test(i) && n % i == 0 {
            res.push(i);
            n /= i;
        }

        i += 1;
    }

    res
}

pub fn dig_list(mut n: usize) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    while n > 0 {
        let curr: u8 = (n % 10) as u8;
        res.push(curr);
        n /= 10;
    }

    res.reverse();
    res
}

pub fn list_dig(vec: &Vec<u8>) -> usize {
    let mut res: usize = 0;
    for i in 0..vec.len() {
        if res >= usize::MAX / 10 {
            println!("{}", res);
        }
        res *= 10;
        res += vec[i] as usize;
    }

    res
}

pub fn is_palindrome<T: std::cmp::PartialEq>(vec: &Vec<T>) -> bool {
    let mut h = vec.len() - 1;
    let mut l = 0;
    while h > l {
        if vec[l] != vec[h] {
            return false;
        }
        h -= 1;
        l += 1;
    }

    true
}

pub fn lcm(x: usize, y: usize) -> usize {
    let mut res: usize;
    let mut l: usize;
    if x > y {
        res = x;
        l = y;
    } else {
        res = y;
        l = x;
    }
    let h = res;

    while res % l != 0 {
        res += h;
    }

    res
}

pub fn segmented_sieve(n: usize) -> Vec<usize> {
    let mut del = (n as f64).sqrt().floor() as usize;
    let mut l: usize = del + 1;
    let mut h: usize = 2 * del; // go from del + 1 to 2*del
    let bv = simple_sieve(del);
    let mut primes: Vec<usize> = Vec::new();

    for i in 2..del {
        if bv[i - 2] {
            primes.push(i);
        }
    }

    let mut first_over: bool = true;
    while h <= n {
        let mut nbv = BitVec::from_elem(del, true);

        let max = (h as f64).sqrt().floor() as usize;
        for &i in &primes {
            if i > max {
                break;
            }
            let mut index_n = (((l as f64) / (i as f64)).ceil() as usize) * i - l;
            while index_n < del {
                nbv.set(index_n, false);
                index_n += i;
            }
        }

        for i in 0..del {
            if nbv[i] {
                primes.push(i + l);
            }
        }

        l = h + 1;
        h += del;

        if h > n && first_over {
            let dif = h - n;
            del -= dif;
            h = n;
            first_over = false;
        }
    }

    primes
}

pub fn dig_listu32(mut n: usize) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::new();
    while n > 0 {
        let curr: u32 = (n % 10) as u32;
        res.push(curr);
        n /= 10;
    }

    res
}

pub fn factors(n: usize) -> Vec<usize> {
    let mut res: Vec<usize> = vec![1];

    for i in 2..((n as f64).sqrt() as usize) {
        if n % i == 0 {
            res.push(i);
            res.push(n / i);
        }
    }

    res.push(n);
    res
}

// assumes elemeents of v are distinct
pub fn combs<T: Copy + Eq>(v: &Vec<T>, n: usize) -> Vec<Vec<T>> {
    let mut res: Vec<Vec<T>> = Vec::new();
    if n == 1 {
        for &i in v {
            res.push(vec![i]);
        }
    } else if v.len() == n {
        res.push(v.clone());
    } else if v.len() > n {
        for i in 0..v.len() {
            let mut tv: Vec<T> = Vec::new();
            for j in (i + 1)..v.len() {
                tv.push(v[j]);
            }
            let nr: Vec<Vec<T>> = combs(&tv, n - 1);
            for mut x in nr {
                x.push(v[i]);
                res.push(x);
            }
        }
    }

    res
}

pub fn prime_variants(n: usize) -> usize {
    let mut max = 0;
    // for every digit we replace, loop over all combs and change, check if prime
    let dl = dig_list(n);
    let places: Vec<usize> = (0..dl.len()).collect();

    for i in 1..(dl.len() + 1) {
        let cmbs = combs(&places, i);
        for cmbvec in &cmbs {
            let mut ct = 0;
            let mut ndl = dl.clone();

            for digit in 0..10 {
                for &place in cmbvec {
                    ndl[place] = digit;
                }
                if ndl[0] == 0 {
                    continue;
                }
                let nn = list_dig(&ndl);
                if naive_prime_test(nn) {
                    ct += 1;
                }
            }

            if ct > max {
                max = ct;
            }
        }
    }

    max
}

pub fn prime_variants_debug(n: usize) -> usize {
    let mut max = 0;
    // for every digit we replace, loop over all combs and change, check if prime
    let dl = dig_list(n);
    let places: Vec<usize> = (0..dl.len()).collect();

    for i in 1..(dl.len() + 1) {
        let cmbs = combs(&places, i);
        for cmbvec in &cmbs {
            let mut ct = 0;
            let mut ndl = dl.clone();
            let mut hash: HashSet<usize> = HashSet::new();

            for digit in 0..10 {
                for &place in cmbvec {
                    ndl[place] = digit;
                }
                if ndl[0] == 0 {
                    continue;
                }
                let nn = list_dig(&ndl);
                if naive_prime_test(nn) {
                    hash.insert(nn);
                    ct += 1;
                }
            }

            if ct > max {
                for i in hash {
                    print!("{} ", i);
                }
                print!(" | ");
                for &place in cmbvec {
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
where
    T: Ord + Copy,
{
    if p > q || p >= r || q >= r {
        return;
    }

    let n1 = q - p + 1;
    let n2 = r - q;
    let mut L: Vec<T> = vec![v[0]; n1 + 1];
    let mut R: Vec<T> = vec![v[0]; n2 + 1];

    for i in 0..n1 {
        L[i] = v[p + i];
    }
    for i in 0..n2 {
        R[i] = v[q + i + 1];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = p;
    while k <= r {
        if i < n1 && (L[i] <= R[j] || j >= n2) {
            v[k] = L[i];
            i += 1;
        } else {
            v[k] = R[j];
            j += 1;
        }

        k += 1;
    }
}

pub fn merge_sort<T>(v: &mut Vec<T>)
where
    T: Ord + Copy,
{
    go_merge(v, 0, v.len() - 1);
}

pub fn go_merge<T>(v: &mut Vec<T>, p: usize, q: usize)
where
    T: Ord + Copy,
{
    if q - p > 0 {
        let m = (((p + q) as f64) / 2.0).floor() as usize;
        go_merge(v, p, m);
        go_merge(v, m + 1, q);
        merge(v, p, m, q);
    }
}

pub fn factorial(n: usize) -> BigUint {
    let mut res = BigUint::from(n);
    for i in 2..n {
        res = res * i;
    }

    res
}

pub fn nchooser(n: usize, r: usize) -> BigUint {
    let mut num: BigUint;
    if r == 1 {
        num = BigUint::from(n);
    } else {
        num = BigUint::from(n);
        for i in ((n - (r - 1))..n).rev() {
            num = num * i;
        }
        let den = factorial(r);
        num = num / den;
    }

    num
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Suit {
    H,
    C,
    S,
    D,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Face {
    Null,
    one,
    two,
    three,
    four,
    five,
    six,
    seven,
    eight,
    nine,
    ten,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Card {
    pub f: Face,
    pub s: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sp = match self.s {
            Suit::H => "H",
            Suit::C => "C",
            Suit::D => "D",
            Suit::S => "S",
        };
        let fp = match self.f {
            Face::Null => "N",
            Face::one => "1",
            Face::two => "2",
            Face::three => "3",
            Face::four => "4",
            Face::five => "5",
            Face::six => "6",
            Face::seven => "7",
            Face::eight => "8",
            Face::nine => "9",
            Face::ten => "T",
            Face::J => "J",
            Face::Q => "Q",
            Face::K => "K",
            Face::A => "A",
        };
        write!(f, "{}{}", fp, sp)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Rank {
    Hc(Face),
    Op(Face),
    Tp(Face, Face),
    Tk(Face),
    S(Face),
    F(Face),
    Fh(Face, Face),
    Fk(Face),
    Sf(Face),
    Rf,
    Sup,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Deck {
    pub cards: [Card; 5],
    pub r: Rank,
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{} {} {} {} {}]",
            self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4]
        )
    }
}

pub fn max_card(d: &Deck) -> i32 {
    d.cards.into_iter().map(|x| x.f as i32).max().unwrap()
}

pub fn get_pairs(d: &Deck) -> Vec<Face> {
    let mut res: Vec<Face> = Vec::new();
    for i in 0..3 {
        for j in (i + 1)..4 {
            if d.cards[i].f == d.cards[j].f {
                let nr = &mut res;
                if !nr.into_iter().any(|&mut x| x == d.cards[i].f) {
                    nr.push(d.cards[i].f);
                }
            }
        }
    }

    res
}

pub fn rank_m(d: &mut Deck, rm: Rank) {
    let mut suit_check = [false; 4];
    let mut rf_ns = [false; 5];
    let mut last_val = d.cards[0].f as i32;
    let mut straight = true;
    let mut vals: Vec<(Face, usize)> = Vec::new();

    for i in 0..5 {
        if d.cards[i].f != Face::Null {
            if i > 0 {
                straight &= d.cards[i].f as i32 - last_val == 1
            }
            last_val = d.cards[i].f as i32;

            let mut inside = false;
            for mut t in &mut vals {
                let (f, _) = t;
                if d.cards[i].f == *f {
                    inside = true;
                    t.1 = t.1 + 1;
                    break;
                }
            }
            if !inside {
                vals.push((d.cards[i].f, 1));
            }

            suit_check[d.cards[i].s as usize] = true;
            if d.cards[i].s as usize >= Face::ten as usize {
                rf_ns[d.cards[i].s as usize - Face::ten as usize] = true;
            }
        }
    }
    let flush: bool = suit_check
        .into_iter()
        .fold(0, |acc, &x| if x { acc + 1 } else { acc })
        == 1;
    let rf_n: bool = rf_ns.into_iter().all(|&x| x);
    let mut max = d.cards[0].f;
    for &i in &d.cards {
        if i.f as usize > max as usize {
            max = i.f;
        }
    }

    let mut max_c: usize = 0;
    let mut max_f: Face = vals[0].0;
    let mut snd_max_c: usize = 0;
    let mut snd_max_f: Face = vals[0].0;
    for (f, c) in &vals {
        if *c > max_c {
            snd_max_f = max_f;
            snd_max_c = max_c;
            max_f = *f;
            max_c = *c;
        } else if *c > snd_max_c {
            snd_max_f = *f;
            snd_max_c = *c;
        }
    }

    if Rank::Rf < rm && rf_n && flush {
        d.r = Rank::Rf;
    } else if Rank::Sf(Face::A) < rm && straight && flush {
        d.r = Rank::Sf(max);
    } else if Rank::Fk(Face::A) < rm && max_c == 4 {
        d.r = Rank::Fk(max_f);
    } else if Rank::Fh(Face::A, Face::A) < rm && max_c == 3 && snd_max_c == 2 {
        d.r = Rank::Fh(max_f, snd_max_f);
    } else if Rank::F(Face::A) < rm && flush {
        d.r = Rank::F(max);
    } else if Rank::S(Face::A) < rm && straight {
        d.r = Rank::S(max);
    } else if Rank::Tk(Face::A) < rm && max_c == 3 {
        d.r = Rank::Tk(max_f);
    } else if Rank::Tp(Face::A, Face::A) < rm && max_c == 2 && vals.len() == 3 {
        d.r = Rank::Tp(max_f, snd_max_f);
    } else if Rank::Op(Face::A) < rm && max_c == 2 {
        d.r = Rank::Op(max_f);
    } else {
        d.r = Rank::Hc(max);
    }
}

pub fn rank(d: &mut Deck) {
    rank_m(d, Rank::Sup);
}

pub fn rank_gt(d1: &mut Deck, d2: &mut Deck) -> bool {
    let mut res = false;
    rank(d1);
    rank(d2);
    loop {
        if d1.r > d2.r {
            res = true;
            break;
        } else if d1.r < d2.r {
            res = false;
            break;
        } else {
            fn iter(d: &mut Deck) {
                match d.r {
                    Rank::Sf(f)
                    | Rank::Fk(f)
                    | Rank::F(f)
                    | Rank::S(f)
                    | Rank::Tk(f)
                    | Rank::Op(f)
                    | Rank::Hc(f) => {
                        for mut i in &mut d.cards {
                            if i.f == f {
                                i.f = Face::Null;
                            }
                        }
                    }
                    Rank::Fh(f1, f2) | Rank::Tp(f1, f2) => {
                        for mut i in &mut d.cards {
                            if i.f == f1 || i.f == f2 {
                                i.f = Face::Null;
                            }
                        }
                    }
                    _ => {}
                }
            }
            iter(d1);
            iter(d2);
            rank_m(d1, d1.r);
            rank_m(d2, d2.r);
        }
    }

    println!(
        "{} {} {} ({:?} vs {:?})",
        &d1,
        if res { "wins" } else { "loses" },
        &d2,
        &d1.r,
        &d2.r
    );
    res
}

pub fn str_to_card(s: String) -> Card {
    let face: Face = match &s[0..1] {
        "1" => Face::one,
        "2" => Face::two,
        "3" => Face::three,
        "4" => Face::four,
        "5" => Face::five,
        "6" => Face::six,
        "7" => Face::seven,
        "8" => Face::eight,
        "9" => Face::nine,
        "T" => Face::ten,
        "J" => Face::J,
        "Q" => Face::Q,
        "K" => Face::K,
        "A" => Face::A,
        _ => {
            println!("{}", &s[0..1]);
            Face::Null
        }
    };

    let suit: Suit = match &s[1..2] {
        "S" => Suit::S,
        "D" => Suit::D,
        "C" => Suit::C,
        "H" => Suit::H,
        _ => Suit::H,
    };

    Card { f: face, s: suit }
}

pub fn is_lychrel(mut n: usize) -> usize {
    let mut count = 1;
    let mut cn = BigUint::from(n);
    loop {
        let os = cn.to_string();
        let os: String = os.graphemes(true).rev().collect();
        let os: BigUint = BigUint::from_str(&os).unwrap();
        cn = cn + os;

        if palindrome::is_palindrome(cn.to_string()) {
            return 0;
        }
        if count == 50 {
            return 1;
        }

        count += 1;
    }
}

pub fn digit_sum_naiive(a: usize, b: u32) -> usize {
    let mut sum: usize = 0;
    let mut n = a.pow(b);
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    n
}

pub fn power_mod(a: usize, b: usize, m: usize) -> usize {
    // ndigits = floor(log_10(a^b)) + 1
    //         = floor(blog_10(a)) + 1
    // digit_sum(n) = sum from k = 1 to ndigits of

    let mut res = 1;
    for i in 0..b {
        res = (res * a) % m;
    }
    res
}

pub fn concat_prime(a: usize, b: usize) -> bool {
    let astr = a.to_string();
    let bstr = b.to_string();
    is_prime((astr.clone() + &bstr).parse().unwrap())
    && is_prime((bstr + &astr).parse().unwrap())
}

pub fn polygonal(sides: usize, n: usize) -> usize {
    let s = sides as i64;
    let n = n as i64;
    ((((s-2) * n * n) - ((s-4)*n)) / 2) as usize
}
