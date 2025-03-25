use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Forth{
    tamanio_pila: u32,
    pila: VecDeque<i16>,
    diccionario: HashMap< String, Vec<String>>,
    flag_literal: bool,
    flag_error: bool,
}



impl Forth{
    pub fn construir(size_in_bytes: u32)-> Forth{
        Forth{
            tamanio_pila: if size_in_bytes == 0{
                131072/2
            } else {size_in_bytes/2},
            pila: VecDeque::new(),
            diccionario: HashMap::new(),
            flag_literal: false,
            flag_error: false,
        }
    }

    pub fn carriage_return(&self){
        println!();
    }

    pub fn leer_linea(&mut self, linea: &str) -> bool{
        let palabras: VecDeque<&str> = linea.split_terminator(&[' '][..]).collect();
        let mut i = 0;
        while i < palabras.len() && !self.flag_error{
            if palabras[i] == ":"{
                i = self.cargar_word(&palabras, i);
            }
            else{
                self.ejecutar_funcion(palabras[i]);
                i += 1;
            }
        }
        self.flag_error
    }

    pub fn ejecutar_funcion(&mut self, funcion: &str){
            if self.flag_literal {
                self.imprimir_literal(funcion);
            }
            match funcion.to_ascii_uppercase().as_str(){
                "CR" => self.carriage_return(),
                "." => self.punto(),
                ".\"" => self.imprimir_literal(funcion),
                "EMIT" => self.emit(),
                "+" => self.sumar(),
                "-" => self.restar(),
                "*" => self.multiplicar(),
                "/" => self.dividir(),
                "DUP" => self.dup(),
                "DROP" => self.drop(),
                "SWAP" => self.swap(),
                "OVER" => self.over(),
                "ROT" => self.rot(),
                "=" => self.igual(),
                ">" => self.mayor(),
                "<" => self.menor(),
                "AND" => self.and(),
                "OR" => self.or(),
                &_ => (),
            }
            match funcion.parse::<i16>() {
                Ok(valor) => self.apilar(valor),
                Err(_) => (),
            }
            let word = self.diccionario.get(funcion);
            match word{
                Some(implementacion) => 
                        for palabra in implementacion.clone(){
                            self.ejecutar_funcion(palabra.as_str());
                        },
                None => (),
            }
    }

    pub fn punto(&mut self){
        let x = self.desapilar();
        print!("{} ", x);      
    }
    
    pub fn cargar_word(&mut self, palabras: &VecDeque<&str>, posicion_inicial: usize) -> usize{
        let mut iterador = posicion_inicial + 2;
        let word = palabras[posicion_inicial+1];
        let mut implementacion: Vec<String> = Vec::new();     
        while palabras[iterador] != ";" && iterador < palabras.len(){
            implementacion.push(palabras[iterador].to_string());
            iterador += 1;
        };
        self.diccionario.insert(word.to_string(), implementacion);
        iterador + 1
    }

    fn imprimir_literal(&mut self, cadena: &str){
        if(!self.flag_literal){
            self.flag_literal = true;
        }
        else{
            let mut cad_string = cadena.to_string();
            match cad_string.pop(){
                Some(val) => if val=='"'{
                                        print!("{cad_string} ");
                                    }else{print!("WRONG");},
                None => (),
            }
            self.flag_literal = false;
        }
    }

    pub fn imprimir_tamanio(&self){
        println!("{}", self.tamanio_pila);
    }

    pub fn imprimir_pila(&self){
        println!("{:?}", self.pila);
    }

    pub fn apilar(&mut self, valor: i16){
        self.pila.push_back(valor);
    }

    pub fn desapilar(&mut self) -> i16{
        if(!self.flag_error){
            match self.pila.pop_back(){
                None => {self.flag_error = true;
                        println!("CHE ESTA VACIA LA PILA PAPA QUE HACES");
                        return 0},
                Some(val) => return val,
            };
        }0
    }

    pub fn sumar(&mut self){
        let val1 = self.desapilar();
        let val2 = self.desapilar();
        self.apilar(val1+val2);
    }

    pub fn restar(&mut self){
        let val1 = self.desapilar();
        let val2 = self.desapilar();
        self.apilar(val2-val1);
    }

    pub fn dividir(&mut self){
        let val1 = self.desapilar();
        let val2 = self.desapilar();
        self.apilar(val2/val1);
    }

    pub fn multiplicar(&mut self){
        let val1 = self.desapilar();
        let val2 = self.desapilar();
        self.apilar(val2*val1);
    }

    pub fn dup(&mut self){
        let val1 = self.desapilar();
        self.apilar(val1);
        self.apilar(val1);

    }

    pub fn drop(&mut self){
        self.desapilar();
    }

    pub fn swap(&mut self){
        let val1 = self.desapilar();
        let val2 = self.desapilar();
        self.apilar(val1);
        self.apilar(val2);

    }

    pub fn over(&mut self){
        let val1 = self.desapilar();
        let val2 = self.desapilar();
        self.apilar(val2);
        self.apilar(val1);
        self.apilar(val2);
    }

    pub fn rot(&mut self){

        let val1 = self.desapilar();
        let val2 = self.desapilar();
        let val3 = self.desapilar();
        self.apilar(val2);
        self.apilar(val1);
        self.apilar(val3);
    }
    
    pub fn emit(&mut self){
        let valor = self.desapilar();
        let mut imprimir: u8 = 127;
        imprimir = imprimir & valor as u8;
        print!("{} ", imprimir as char);
    }

    pub fn mayor(&mut self){
        let val1 = self.desapilar();
        let val2 = self.desapilar();
        if val2>val1 {self.pila.push_back(-1)}
        else{self.pila.push_back(0);}
    }

    pub fn menor(&mut self){
        let val1 = self.desapilar();
        let val2 = self.desapilar();
        if val2<val1 {self.pila.push_back(-1)}
        else{self.pila.push_back(0);}
    }

    pub fn igual(&mut self){
        let val1 = self.desapilar();
        let val2 = self.desapilar();
        if val2==val1 {self.pila.push_back(-1)}
        else{self.pila.push_back(0);}
    }

    pub fn and(&mut self){
        let val1 = self.desapilar();
        let val2 = self.desapilar();
        self.apilar(val2&val1);
    }

    pub fn or(&mut self){
        let val1 = self.desapilar();
        let val2 = self.desapilar();
        self.apilar(val2 | val1);
    }
}

