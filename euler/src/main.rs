mod probs;
mod accs;

fn main() 
{
    let i = accs::suit::H;
    let j = accs::face::J;
    println!("{} {}", i as i32, j as i32);
    //println!("{}", probs::prob54());
}
