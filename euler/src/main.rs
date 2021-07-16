mod probs;

fn main() 
{
    println!("{}", vec![2, 3, 21, 44, 103].map(|x| probs::naiive_prime_test(x));
}
