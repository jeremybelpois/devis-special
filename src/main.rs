use std::env;
use rand::{thread_rng, Rng};

const DEVIS_FR: [&str; 3] = [
	"Aelita, nous avons gagné. XANA a été vaincu !",
	"Ce monde virtuel, c’est tout un écosystème, avec des êtres vivants artificiels !",
	"Toi, entrer là-d’dans et toi être téléporté dans l’monde virtuel !",
];

const DEVIS_EN: [&str; 3] = [
	"A supercomputer in a factory? A virtual world?",
	"Welcome to the virtual world.",
	"You’re in a virtual world created by XANA.",
];

fn main() {
	let args: Vec<String> = env::args().collect();
	let devis = match args.len() {
		2 => {
			match args[1].as_ref() {
				"en" => &DEVIS_EN,
				_ => &DEVIS_FR,
			}
		},
		_ => &DEVIS_FR,
	};

	if args.len() < 2 {
		println!("Content-type: text/html\n");
	}

	println!("{}", devis[thread_rng().gen_range(0, devis.len())]);
}
