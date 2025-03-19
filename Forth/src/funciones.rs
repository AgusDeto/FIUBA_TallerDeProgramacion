
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

    pub fn punto(&mut self){
        let x = self.pila.pop();
        match x{
            None => (),//error
            Some(val) => println!("{} ", val),
        }
    }

    pub fn imprimir_tamanio(&self){
        println!("{}", self.tamanio_pila);
    }

    pub fn imprimir_pila(&self){
        println!("{:?}", self.pila);
    }

    pub fn apilar(&mut self, valor: &str){
        self.pila.push(valor.parse::<i16>().unwrap());
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
    /* 
    pub fn emit(){
        match self.pila.pop(){
            None => (),//error
            Some(val) => print('{val}'),
        }
    }*/
    pub fn mayor(&mut self){
        match self.pila.pop(){
            None => (),
            Some(val1) => {
                match self.pila.pop(){
                None => (),
                Some(val2) => {
                    if(val2>val1){self.pila.push(-1)}
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
                    if(val2<val1){self.pila.push(-1)}
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
                    if(val2==val1){self.pila.push(-1)}
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

