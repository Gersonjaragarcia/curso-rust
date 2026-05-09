fn main() {
    let nombre = String::from("Juan");

    let dato = nombre;

    println!("{}", dato);

    let num = "Mario";

    println!("{}", num);

    let mut edad = 18;

    edad = 20;

    println!("{}", edad);

    // Tuplas
    let persona = ("Carlos", "Maria", "Veronica", 30, true);

    println!("{}", persona.1);

    // Arrays
    let numeros = [1, 2, 3, 4, 5, 6, 7];

    println!("{}", numeros[0]);

    // Conversión de tipos
    let numero = 30;

    let decimal = numero as f64;

    println!("{}", decimal);

    // opraciones matematicas
    let  suma= 10 + 20;

    let resta = 30 - 20;

    let multiplicacion = 45*2;

    let division= 10/5;

    let potenciacion= 3_i32.pow(2); 


    println!("La suma es: {}",suma);

    println!("La resta es: {}",resta);

    println!("La multiplicacion es: {}",multiplicacion);

    println!("La division es: {}",division);

    println!("La potenciacion es: {}",potenciacion);

    // condicionales
    let num1= 20; 

    let  num1 = 15;

    if num1 >= 10{
        println!(" el numero es mayor")
    }

    if num1 >= 40{
        println!("Es mayor");
    } else {
        println!("Es menor")
    }

    let nota = 85; 

    if nota >= 90{
        println!("Excelente");
    }else if nota  >= 70 {
        println!("Aprovado");

        
    }else {
        println!("Reprobado")
    }

    // Operadores Logicos
    let  edad =30;
    let tiene_id = true; 

    if edad >=20 && tiene_id{
        println!("Puede entrar");
    }

    let admin = false; 
    let moderador=true;

    if admin || moderador {
        println!("Acceso permitido");
    }

    // if como expresion
    let edad = 20; 

    let mensaje = if edad >=10{
        "Mayor"
    }else {
        "Menor"
    };

    println!("{}", mensaje);

    // match
    let numero = 2;


    match numero {

        1 => println!("Uno"),
        2 => println!("Dos"),
        3 => println!("Tres"),
        4 => println!("Cuatro"),
        5 => println!("Cinco"),
        _ => println!("Otro número")
        
    }

}