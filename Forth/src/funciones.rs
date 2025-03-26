use std::collections::VecDeque;
mod funcion;
use crate::funciones::funcion::Funcion;
mod word_manager;
use crate::funciones::word_manager::WordManager;

pub struct Forth{
    tamanio_pila: usize,
    pila: VecDeque<i16>,
    word_manager: WordManager,
    flag_literal: bool,
    flag_error: bool,
    flag_if: (bool,bool),
}



impl Forth{
    pub fn construir(size_in_bytes: usize)-> Forth{
        let c = Forth{
            tamanio_pila: if size_in_bytes == 0{
                131072/2
            } else {size_in_bytes/2},
            pila: VecDeque::new(),
            word_manager: WordManager::crear(),
            flag_literal: false,
            flag_error: false,
            flag_if: (false,false),
        };
        c
    }

    pub fn leer_linea(&mut self, linea: &str) -> bool{
        let mut palabras: VecDeque<&str> = linea.split_terminator(&[' '][..]).collect();
        let mut ejecutador = Funcion::crear();
        while !palabras.is_empty() && !self.flag_error{
            if palabras[0] == ":"{
                palabras.pop_front();
                let word_name = palabras.pop_front().unwrap();
                let mut word_instructions: String = String::new();
                while palabras[0] != ";" {
                    word_instructions.push_str(palabras[0]);
                    word_instructions.push(':');
                    palabras.pop_front();
                }
                self.word_manager.agregar_word(word_name.to_string(), word_instructions);
            }
            else{
                ejecutador.ejecutar_funcion(self, palabras[0]);
            }
            palabras.pop_front();
        }
        self.flag_error
    }

    pub fn contiene_word(&mut self, word: &str) -> bool{
        self.word_manager.contiene_word(word.to_string())
    }

    pub fn ejecutar_word(&mut self, word: &str){
            let mut ejecutador = Funcion::crear();
            let instrucciones_string = self.word_manager.obtener_instrucciones(word.to_string());
            let instrucciones: Vec<&str> = instrucciones_string.split_terminator(&[':'][..]).collect();
            for instruccion in instrucciones{
                if instruccion == word {
                    ejecutador.ejecutar_funcion_base(self, instruccion);
                }
                else{
                    ejecutador.ejecutar_funcion(self, instruccion);
                }

            }
        }
    

    pub fn apilar(&mut self, valor: i16){
        if self.pila.len() == self.tamanio_pila{
            self.flag_error = true;
            println!("CHE ESTA LLENA LA PILA PAPA QUE HACES");
        }
        else{
            self.pila.push_back(valor);
        }
    }

    pub fn desapilar(&mut self) -> i16{
        if !self.flag_error {
            match self.pila.pop_back(){
                None => {self.flag_error = true;
                        println!("CHE ESTA VACIA LA PILA PAPA QUE HACES");
                        return 0},
                Some(val) => return val,
            };
        }0
    }

    
}

