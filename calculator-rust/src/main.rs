use std::io;

fn main() {
    println!("Ingrese la operacion +, -, /, *: ");

    let mut oper = String::new();
    io::stdin().read_line(&mut oper).expect("Error al leer oper");
    
    //Convierto oper en sting y quito los espacios
    let dato_oper: String = oper.trim().to_string();

    println!("Ingrese primer numero: ");
    let mut num1 = String::new();

    io::stdin().read_line(&mut num1).expect("Error al leer primer numero");


    println!("Operacion es {}, y el numero es {}", dato_oper, num1);


}
