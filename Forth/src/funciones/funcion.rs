

pub struct Funcion{
}

impl Funcion{
    pub fn crear() -> Funcion{
        Funcion{
            
        }
    }

    pub fn ejecutar_funcion_base(&mut self, pila: &mut super::Forth, funcion: &str){
        match funcion.to_ascii_uppercase().as_str(){
                "CR" => self.carriage_return(),
                "." => self.punto(pila),
                ".\"" => self.imprimir_literal(pila, funcion),
                "EMIT" => self.emit(pila),
                "+" => self.sumar(pila),
                "-" => self.restar(pila),
                "*" => self.multiplicar(pila),
                "/" => self.dividir(pila),
                "DUP" => self.dup(pila),
                "DROP" => self.drop(pila),
                "SWAP" => self.swap(pila),
                "OVER" => self.over(pila),
                "ROT" => self.rot(pila),
                "=" => self.igual(pila),
                ">" => self.mayor(pila),
                "<" => self.menor(pila),
                "AND" => self.and(pila),
                "OR" => self.or(pila),
                "IF" => self.comienzo_if(pila),
                &_ => (),
            }
            if let Ok(valor) = funcion.parse::<i16>() {
                pila.apilar(valor) 
            }
        }
    

    pub fn ejecutar_funcion(&mut self, pila: &mut super::Forth, funcion: &str){
        if pila.contiene_word(funcion){
            pila.ejecutar_word(funcion);
        } else if pila.flag_literal {
            self.imprimir_literal(pila, funcion);
        } else if pila.flag_if.0 { 
            self.ejecutar_if(pila, funcion);
        } else{
            self.ejecutar_funcion_base(pila, funcion);
        }
    }

    pub fn carriage_return(&self){
        println!();
    }

    pub fn punto(&mut self, pila: &mut super::Forth){
        let x = pila.desapilar();
        print!("{} ", x);      
    }

    pub fn imprimir_literal(&mut self, pila: &mut super::Forth, cadena: &str){
        if !pila.flag_literal {
            pila.flag_literal = true;
        }
        else{
            let mut cad_string = cadena.to_string();
            if let Some(val) = cad_string.pop() { 
                if val=='"' {
                    print!("{cad_string} ");
                }else{
                    print!("WRONG");
                } 
            }
        pila.flag_literal = false;
        }
    }

    pub fn sumar(&mut self, pila: &mut super::Forth){
        let val1 = pila.desapilar();
        let val2 = pila.desapilar();
        pila.apilar(val1+val2);
    }

    pub fn restar(&mut self, pila: &mut super::Forth){
        let val1 = pila.desapilar();
        let val2 = pila.desapilar();
        pila.apilar(val2-val1);
    }

    pub fn dividir(&mut self, pila: &mut super::Forth){
        let val1 = pila.desapilar();
        let val2 = pila.desapilar();
        if val1 == 0{
            pila.flag_error = true;
            println!("WTF COMO VAS A DIVIDIR POR 0 MAN");
        }
        else{
            pila.apilar(val2 / val1);
        }
    }

    pub fn multiplicar(&mut self, pila: &mut super::Forth){
        let val1 = pila.desapilar();
        let val2 = pila.desapilar();
        pila.apilar(val2*val1);
    }

    pub fn dup(&mut self, pila: &mut super::Forth){
        let val1 = pila.desapilar();
        pila.apilar(val1);
        pila.apilar(val1);

    }

    pub fn drop(&mut self, pila: &mut super::Forth){
        pila.desapilar();
    }

    pub fn swap(&mut self, pila: &mut super::Forth){
        let val1 = pila.desapilar();
        let val2 = pila.desapilar();
        pila.apilar(val1);
        pila.apilar(val2);

    }

    pub fn over(&mut self, pila: &mut super::Forth){
        let val1 = pila.desapilar();
        let val2 = pila.desapilar();
        pila.apilar(val2);
        pila.apilar(val1);
        pila.apilar(val2);
    }

    pub fn rot(&mut self, pila: &mut super::Forth){

        let val1 = pila.desapilar();
        let val2 = pila.desapilar();
        let val3 = pila.desapilar();
        pila.apilar(val2);
        pila.apilar(val1);
        pila.apilar(val3);
    }
    
    pub fn emit(&mut self, pila: &mut super::Forth){
        let valor = pila.desapilar();
        let mut imprimir: u8 = 127;
        imprimir &= valor as u8;
        print!("{} ", imprimir as char);
    }

    pub fn mayor(&mut self, pila: &mut super::Forth){
        let val1 = pila.desapilar();
        let val2 = pila.desapilar();
        if val2>val1 {pila.pila.push_back(-1)}
        else{pila.pila.push_back(0);}
    }

    pub fn menor(&mut self, pila: &mut super::Forth){
        let val1 = pila.desapilar();
        let val2 = pila.desapilar();
        if val2<val1 {pila.pila.push_back(-1)}
        else{pila.pila.push_back(0);}
    }

    pub fn igual(&mut self, pila: &mut super::Forth){
        let val1 = pila.desapilar();
        let val2 = pila.desapilar();
        if val2==val1 {pila.pila.push_back(-1)}
        else{pila.pila.push_back(0);}
    }

    pub fn and(&mut self, pila: &mut super::Forth){
        let val1 = pila.desapilar();
        let val2 = pila.desapilar();
        pila.apilar(val2&val1);
    }

    pub fn or(&mut self, pila: &mut super::Forth){
        let val1 = pila.desapilar();
        let val2 = pila.desapilar();
        pila.apilar(val2 | val1);
    }

    pub fn comienzo_if(&mut self,pila: &mut super::Forth){
        if !pila.flag_if.0 {
            pila.flag_if.0 = true;
            pila.flag_if.1 = pila.desapilar() != 0;
        }
    }

    pub fn ejecutar_if(&mut self,pila: &mut super::Forth, funcion: &str){
        if funcion == "THEN" {
            pila.flag_if = (false,false);
        } else if funcion == "ELSE" && !pila.flag_if.1 {
            pila.flag_if.1 = true;
        } else if funcion == "ELSE"{
            pila.flag_if.1 = false;
        } else if pila.flag_if.1 {
            self.ejecutar_funcion_base(pila, funcion);
        }
    }
}