extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main()
{
	println!("Vamos a adivinar!");

	let numero_elegido = rand::thread_rng().gen_range(1, 101);

	loop{

	println!("Ingresa un numero del 1 al 100");

	let mut adi = String::new();

	io::stdin().read_line(&mut adi)
		.ok()
		.expect("Fallo al leer la linea");

	let adi: u32 = match adi.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
        };

	println!("Tu numero fue: {}", adi);

	match adi.cmp(&numero_elegido)
	{
		Ordering::Less => println!("Muy pequenio"),
		Ordering::Greater => println!("Muy grande choquito"),
		Ordering::Equal => {
		println!("Le achuntaste caramba");
		break;
	}
	}
	}
}
