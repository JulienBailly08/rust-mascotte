use ferris_says::say;
use std::io;
use std::io::{stdout, BufWriter};

fn main() {
    let mut first = true;
    loop {
        if first {
            println!("Que va dire Ferry ?");
            first = false;
        } else {
            println!("Et maintenant que va-t-il dire ?");
        }

        let mut texte = String::new();

        io::stdin()
            .read_line(&mut texte)
            .expect("Failed to read line");

        let width = texte.bytes().count();

        let mut writer = BufWriter::new(stdout());
        say(&texte, width, &mut writer).unwrap();
        println!();
    }
}
