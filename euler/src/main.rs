mod probs;

fn main() 
{
    let v = vec![1,2,3,4];
    for i in probs::combs(&v, 3)
    {
        for j in i
        {
            print!("{} ", j);
        }
        println!("");
    }
}
