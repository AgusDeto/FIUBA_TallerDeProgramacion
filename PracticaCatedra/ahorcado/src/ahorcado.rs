use std::io::{stdout, Write};

#[derive(Debug)]
pub enum InputError{
    MasDeUnCaracter,
    NoEsUnaLetraMinuscula,
    LetraYaIngresada,
}
struct Ronda{
    intentos: i8,    
    palabra_tapada: Vec<char>,
    letras_usadas: Vec<char>,
}

impl Ronda{
    fn contruir_ronda(len_vec: usize) -> Ronda{
        Ronda{
            intentos: 5,
            palabra_tapada: vec!['_'; len_vec],
            letras_usadas: Vec::new(),
        }
    }

    fn victoria(&mut self) -> bool{
        print!("Felicidades, encontraste la palabra ");
        for letra in self.palabra_tapada.clone(){
            print!("{}", letra);
        }
        println!();
        true
    }

    fn derrota(&mut self) -> bool{
        println!("Nice try, you lose! ");
        false
    }

    fn imprimir_estado_ronda(&self){
        print!("\nLa palabra hasta el momento es: ");
        for letra in self.palabra_tapada.clone(){
            print!("{} ", letra);
        }
        print!("\nUsaste las siguiente letras: ");
        for letra in self.letras_usadas.clone(){
            print!("{} ", letra);
        }
        if self.intentos > 0{
            println!("\nTe quedan {} intentos.", self.intentos);
            print!("Ingresa una letra: ");
            stdout().flush().expect("No se pudo flushear.");
        }
    }
}


type InputResult = Result<char, InputError>;

fn ingreso_letra(letras_usadas: &[char]) -> InputResult{
    let mut letra = String::new();
    let bytes = std::io::stdin().read_line(&mut letra).unwrap_or(0);
    let caracter = letra.chars().next().unwrap_or('0');
    if bytes > 2 {
        Err(InputError::MasDeUnCaracter)
    }else if !caracter.is_ascii_lowercase(){
        Err(InputError::NoEsUnaLetraMinuscula)
    }else if letras_usadas.contains(&caracter){
        Err(InputError::LetraYaIngresada)
    }else{
        Ok(caracter)
    }
}

fn lectura_letra(letras_usadas: &[char]) -> char{
    let mut caracter = ingreso_letra(letras_usadas);
    while caracter.is_err() {
        if let Err(_err) = caracter{
            print!("Ingreso invalido, vuelva a intentarlo ({:?}): ", _err);
        }
        stdout().flush().expect("No se pudo flushear.");
        caracter = ingreso_letra(letras_usadas);
    }
    caracter.unwrap()
}


fn adivinar_palabra(palabra: String) -> bool{

    let mut palabra_transformada = palabra.chars().collect::<Vec<char>>();
    let mut ronda = Ronda::contruir_ronda(palabra_transformada.len());
    while ronda.intentos > 0 {
        ronda.imprimir_estado_ronda();
        let letra = lectura_letra(&ronda.letras_usadas);
        let mut posicion = palabra_transformada.iter().position( |&x| x == letra);
        if posicion.is_none(){
            ronda.intentos -= 1;
        }
        else{ while posicion.is_some() {
            palabra_transformada[posicion.unwrap()] = '_';
            ronda.palabra_tapada[posicion.unwrap()] = letra;
            posicion = palabra_transformada.iter().position(|&x| x == letra);
        }}
        if !ronda.palabra_tapada.iter().any(|&x| x == '_'){
            return ronda.victoria(); 
        }
        ronda.letras_usadas.push(letra);
    }
    ronda.derrota()
}


pub fn inicio_ahorcado(palabras: String){
    println!("Bienvenido al ahorcado de FIUBA!:");
    let mut continuar = true;
    for linea in palabras.lines(){
        if continuar{
            continuar = adivinar_palabra(linea.to_string());
        }
    }
}