use std::io;

fn main() {
    println!("Ingrese la operacion +, -, /, *: ");

    let mut oper = String::new();
    io::stdin().read_line(&mut oper).expect("Error al leer oper");
    
    //Convierto oper en sting y quito los espacios
    let dato_oper: String = oper.trim().to_string();

    println!("Ingrese primer numero: ");
    let mut num1: String  = String::new();

    io::stdin().read_line(&mut num1).expect("Error al leer primer numero");

    println!("Ingrese segundo numero: ");
    let mut num2: String = String::new();

    io::stdin().read_line(&mut num2).expect("Error al leer segundo numero");

    let num1: i32 = num1.trim().parse().expect("Error al convertir primer numero");
    let num2: i32 = num2.trim().parse().expect("Error al convertir Segundo numero");


    if dato_oper == "+" {
        let result: i32 = num1 + num2;
        println!("------------------------------------");
        println!("El resultado de la operacion es: {}", result);
    }

    match dato_oper.as_str() {  
        "+" => {
            let result: i32 = num1 + num2;
            println!("------------------------------------");
            println!("El resultado de la operacion es: {}", result);
        }
        "-" => {
            let result: i32 = num1 - num2;
            println!("------------------------------------");
            println!("El resultado de la operacion es: {}", result);
        }
        "/" => {
            let result: i32 = num1 / num2;
            println!("------------------------------------");
            println!("El resultado de la operacion es: {}", result);
        }
        "*" => {
            let result: i32 = num1 * num2;
            println!("------------------------------------");
            println!("El resultado de la operacion es: {}", result);
        }
        
        _ => {
            println!("Operacion no valida")
        }
    }
}
