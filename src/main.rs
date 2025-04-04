use fake::{Fake, faker::company::en::CatchPhrase};

fn main() {
    let title: String = CatchPhrase().fake();
    let description: String = format!(
        "{}, {}, {}, {}",
        CatchPhrase().fake::<String>(),
        CatchPhrase().fake::<String>(),
        CatchPhrase().fake::<String>(),
        CatchPhrase().fake::<String>()
    );

    println!("{title}\n{description}");
}
