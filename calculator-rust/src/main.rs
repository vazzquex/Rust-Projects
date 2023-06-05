use std::io;

fn main() {
    println!("Ingrese la operacion +, -, /, *: ");

    let mut oper = String::new();

    if let Err(error) = io::stdin().read_line(&mut oper) {
        eprintln!("Error al leer la entrada: {}", error);
        return;
    }

    let dato_oper: String = oper.trim().to_string();

    println!("Ingrese primer numero: ");
    let mut num1 = String::new();

    if let Err(error) = io::stdin().read_line(&mut num1) {
        eprintln!("Error: {}", error);
        return;
    }

    println!("Operacion es {}, y el numero es {}", dato_oper, num1);


}
