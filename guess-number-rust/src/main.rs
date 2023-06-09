use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("¡Bienvenido al juego de adivinación!");

    let secreto = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Por favor, introduce tu predicción.");

        let mut predicción = String::new();

        io::stdin().read_line(&mut predicción)
            .expect("Falló al leer la línea");

        let predicción: u32 = match predicción.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("¡Por favor, introduce un número!");
                continue;
            }
        };

        println!("Tu predicción: {}", predicción);

        match predicción.cmp(&secreto) {
            Ordering::Less => println!("¡Muy bajo!"),
            Ordering::Greater => println!("¡Muy alto!"),
            Ordering::Equal => {
                println!("¡Acertaste!");
                break;
            }
        }
    }
}
