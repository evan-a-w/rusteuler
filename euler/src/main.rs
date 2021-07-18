mod probs;
mod accs;

fn main() 
{
    let mut deck = accs::Deck {
        cards: [accs::Card {f: accs::Face::five, s: accs::Suit::H},
                accs::Card {f: accs::Face::five, s: accs::Suit::C},
                accs::Card {f: accs::Face::six, s: accs::Suit::S},
                accs::Card {f: accs::Face::K, s: accs::Suit::D},
                accs::Card {f: accs::Face::seven, s: accs::Suit::S}],
        r: accs::Rank::Rf,
    };
    accs::rank(&mut deck);
    println!("{:?}", deck.r);

    let mut deck = accs::Deck {
        cards: [accs::Card {f: accs::Face::three, s: accs::Suit::D},
                accs::Card {f: accs::Face::six, s: accs::Suit::D},
                accs::Card {f: accs::Face::seven, s: accs::Suit::D},
                accs::Card {f: accs::Face::ten, s: accs::Suit::D},
                accs::Card {f: accs::Face::Q, s: accs::Suit::D}],
        r: accs::Rank::Rf,
    };
    accs::rank(&mut deck);
    println!("{:?}", deck.r);

    let mut deck = accs::Deck {
        cards: [accs::Card {f: accs::Face::two, s: accs::Suit::H},
                accs::Card {f: accs::Face::two, s: accs::Suit::D},
                accs::Card {f: accs::Face::four, s: accs::Suit::C},
                accs::Card {f: accs::Face::four, s: accs::Suit::D},
                accs::Card {f: accs::Face::four, s: accs::Suit::S}],
        r: accs::Rank::Rf,
    };
    accs::rank(&mut deck);
    println!("{:?}", deck.r);
    //println!("{}", probs::prob54());
}
