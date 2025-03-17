use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
//use std::fs::read_to_string;

//precond: Debe recibir un string referente a una ruta de archivo
//postcond: Devuelve en un String el contenido del archivo o el motivo de la falla en caso de error 
pub fn lectura_archivo(filepath: &str) -> String{
    let ruta = Path::new(filepath);

    let mut archivo_palabras = match File::open(ruta){
        Err(_io_error) => return "io_error apertura de archivo".to_string(),
        Ok(archivo_palabras) => archivo_palabras,
    };
    
    let mut s = String::new();
    match archivo_palabras.read_to_string(&mut s){
        Err(_io_error) => "io_error lectura de archivo".to_string(),
        Ok(_) => s,
    }
}

/*
fn lectura_por_linea(filename: &str) -> Vec<String> {
    read_to_string(filename).unwrap().lines().map(String::from).collect() 
}*/