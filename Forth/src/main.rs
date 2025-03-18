use std::{env, num::ParseIntError};
mod funciones;
use crate::funciones::Forth;

fn leer_stack_size(arg_stack_size: &String) -> Result<u32, ParseIntError>{
    let vec = arg_stack_size.chars();
    let mut numero: String = String::new();
    for caracter in vec{
        if caracter.is_numeric(){
            numero.push(caracter);
        }
    }
    numero.parse::<u32>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let _ruta = &args[1];
    let mut stack_size = 0;
    if args.len() == 3{
        stack_size = match leer_stack_size(&args[2]){
            Ok(tamaño) => tamaño,
            Err(_) => 0,
        };
    }
    println!("{stack_size}");
    let mut forth = Forth::construir(stack_size);
    forth.imprimir_tamanio();
    forth.carriage_return();
    forth.apilar("25");
    forth.apilar("10");
    forth.imprimir_pila();
    forth.sumar();
    forth.imprimir_pila();
    forth.apilar("3");
    forth.multiplicar();
    forth.imprimir_pila();
    forth.punto();
    forth.punto();

    
}