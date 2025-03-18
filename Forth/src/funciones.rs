
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
        let mut suma: i16 = 0;
        let x = self.pila.pop();
        match x{
            None => (),//error
            Some(val) => suma += val,
        }
        let y = self.pila.pop();
        match y{
            None => (),//error
            Some(val) => suma += val,
        }
        self.pila.push(suma);
    }

    pub fn restar(&mut self){
        let mut resta: i16 = 0;
        let x = self.pila.pop();
        match x{
            None => (),//error
            Some(val) => resta += val,
        }
        let y = self.pila.pop();
        match y{
            None => (),//error
            Some(val) => resta -= val,
        }
        self.pila.push(resta);
    }

    pub fn dividir(&mut self){
        let mut division: i16 = 0;
        let x = self.pila.pop();
        match x{
            None => (),//error
            Some(val) => division += val,
        }
        let y = self.pila.pop();
        match y{
            None => (),//error
            Some(val) => division /= val,
        }
        self.pila.push(division);
    }

    pub fn multiplicar(&mut self){
        let mut multiplicacion: i16 = 0;
        let x = self.pila.pop();
        match x{
            None => (),//error
            Some(val) => multiplicacion += val,
        }
        let y = self.pila.pop();
        match y{
            None => (),//error
            Some(val) => multiplicacion *= val,
        }
        self.pila.push(multiplicacion);
    }
}
