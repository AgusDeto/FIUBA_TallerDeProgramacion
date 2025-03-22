use std::{env, num::ParseIntError};
mod funciones;
use crate::funciones::Forth;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

fn leer_stack_size(arg_stack_size: &str) -> Result<u32, ParseIntError>{
    let vec = arg_stack_size.chars();
    let mut numero: String = String::new();
    for caracter in vec{
        if caracter.is_numeric(){
            numero.push(caracter);
        }
    }
    numero.parse::<u32>()
}

fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let ruta = &args[1];
    let mut stack_size = 0;
    if args.len() == 3{
        stack_size = match leer_stack_size(&args[2]){
            Ok(tamaño) => tamaño,
            Err(_) => 0,
        };
    }
    let f = File::open(ruta)?;
    let f = BufReader::new(f);
    
    let mut compilador: Forth = Forth::construir(stack_size);

    for line in f.lines() {
        let line = line?;
        compilador.leer_linea(&line);
    }

    Ok(())
}