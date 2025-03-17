use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

type Diccionario = HashMap<String, i16>;

fn contar_linea(linea: &str, diccionario: &mut Diccionario){
    let palabras: Vec<&str> = linea.split_terminator(&['.', ':',',',' ',';'][..]).collect();
    for palabra in palabras{
        if !palabra.is_empty(){
            let palabra_minuscula = palabra.to_lowercase();
            if let Some(x) = diccionario.get_mut(&palabra_minuscula) {
                *x += 1;
            } else{
                diccionario.insert(palabra_minuscula, 1);
            }
        }
    }
}

fn imprimir_en_orden(dicc: &Diccionario){
    let mut vec: Vec<(&String, &i16)> = dicc.iter().collect();
    vec.sort_by(|a,b| b.1.cmp(a.1));
    for elemento in vec{
        println!("{} -> {}", elemento.0, elemento.1);
    }
}


fn main() -> io::Result<()> {
    let f = File::open("texto.txt")?;
    let f = BufReader::new(f);
    let mut diccionario: Diccionario = HashMap::new();

    for line in f.lines() {
        let line = line?;
        contar_linea(&line, &mut diccionario);
    }

    imprimir_en_orden(&diccionario);

    Ok(())
}
