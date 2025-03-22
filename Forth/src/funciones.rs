
pub struct Forth{
    tamanio_pila: u32,
    pila: Vec<i16>,
}

impl Forth{
    pub fn construir(size_in_bytes: u32)-> Forth{
        Forth{
            tamanio_pila: if size_in_bytes == 0{
                131072/2
            } else {size_in_bytes/2},
            pila: Vec::new(),
        }
    }

    pub fn carriage_return(&self){
        println!();
    }

    pub fn leer_linea(&mut self, linea: &str){
        let palabras: Vec<&str> = linea.split_terminator(&[' '][..]).collect();
        for palabra in palabras{
            self.ejecutar_funcion(palabra);
        }
    }

    pub fn ejecutar_funcion(&mut self, funcion: &str){
        match funcion{
            "CR" => self.carriage_return(),
            "." => self.punto(),
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
    }

    pub fn punto(&mut self){

        let x = self.pila.pop();
        match x{
            None => println!("Stack underflow"),
            Some(val) => print!("{} ", val),
        }
    }

    fn imprimir_literal(cadena: &str){
        
    }

    pub fn imprimir_tamanio(&self){
        println!("{}", self.tamanio_pila);
    }

    pub fn imprimir_pila(&self){
        println!("{:?}", self.pila);
    }

    pub fn apilar(&mut self, valor: i16){
        self.pila.push(valor);
    }

    pub fn sumar(&mut self){
        match self.pila.pop(){
            None => (),//error
            Some(val1) => {
                match self.pila.pop(){
                    None => (),//error
                    Some(val2) => self.pila.push(val1+val2),
                }
            },
        }
    }

    pub fn restar(&mut self){
        match self.pila.pop(){
            None => (),//error
            Some(val1) => {
                match self.pila.pop(){
                    None => (),//error
                    Some(val2) => self.pila.push(val2-val1),
                }
            },
        }
    }

    pub fn dividir(&mut self){
        match self.pila.pop(){
            None => (),//error
            Some(val1) => {
                match self.pila.pop(){
                    None => (),//error
                    Some(val2) => self.pila.push(val2/val1),
                }
            },
        }
    }

    pub fn multiplicar(&mut self){
        match self.pila.pop(){
            None => (),//error
            Some(val1) => {
                match self.pila.pop(){
                    None => (),//error
                    Some(val2) => self.pila.push(val1*val2),
                }
            },
        }
    }

    pub fn dup(&mut self){
        match self.pila.pop(){
            None => (),//error
            Some(val) => 
                {self.pila.push(val);
                self.pila.push(val)},
        }
    }

    pub fn drop(&mut self){
        match self.pila.pop(){
            None => (),//error
            Some(_) => (),
        }
    }

    pub fn swap(&mut self){
        match self.pila.pop(){
            None => (),
            Some(val1) => {
                match self.pila.pop(){
                None => (),
                Some(val2) => {
                    self.pila.push(val1);
                    self.pila.push(val2);
                },
            }},
        }
    }

    pub fn over(&mut self){
        match self.pila.pop(){
            None => (),
            Some(val1) => {
                match self.pila.pop(){
                None => (),
                Some(val2) => {
                    self.pila.push(val2);
                    self.pila.push(val1);
                    self.pila.push(val2);
                },
            }},
        }
    }

    pub fn rot(&mut self){
        match self.pila.pop(){
            None => (),
            Some(val1) => {
                match self.pila.pop(){
                None => (),
                Some(val2) => match self.pila.pop(){
                    None => (),
                    Some(val3) => {
                        self.pila.push(val2);
                        self.pila.push(val1);
                        self.pila.push(val3);
                    },}
            }},
        }
    }
    
    pub fn emit(&mut self){
        let mut imprimir: u8 = 127;
        match self.pila.pop(){
            None => (),//error
            Some(val) => {imprimir = imprimir & val as u8;
                            print!("{} ", imprimir as char)},
        }
    }

    pub fn mayor(&mut self){
        match self.pila.pop(){
            None => (),
            Some(val1) => {
                match self.pila.pop(){
                None => (),
                Some(val2) => {
                    if val2>val1 {self.pila.push(-1)}
                    else{self.pila.push(0);}
                },
            }},
        }
    }

    pub fn menor(&mut self){
        match self.pila.pop(){
            None => (),
            Some(val1) => {
                match self.pila.pop(){
                None => (),
                Some(val2) => {
                    if val2<val1 {self.pila.push(-1)}
                    else{self.pila.push(0);}
                },
            }},
        }
    }

    pub fn igual(&mut self){
        match self.pila.pop(){
            None => (),
            Some(val1) => {
                match self.pila.pop(){
                None => (),
                Some(val2) => {
                    if val2==val1 {self.pila.push(-1)}
                    else{self.pila.push(0);}
                },
            }},
        }
    }

    pub fn and(&mut self){
        match self.pila.pop(){
            None => (),
            Some(val1) => {
                match self.pila.pop(){
                None => (),
                Some(val2) => self.pila.push(val1 & val2),
                }
            },
        }
    }

    pub fn or(&mut self){
        match self.pila.pop(){
            None => (),
            Some(val1) => {
                match self.pila.pop(){
                None => (),
                Some(val2) => self.pila.push(val1 | val2),
                }
            },
        }
    }
}

