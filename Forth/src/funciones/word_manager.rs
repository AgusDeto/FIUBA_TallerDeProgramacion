use std::collections::HashMap;

pub struct WordManager{
    diccionario: HashMap< String, String>,
}
impl WordManager{
    pub fn crear() -> WordManager{
        WordManager{
            diccionario: HashMap::new(),
        }
    }

    pub fn agregar_word(&mut self, word_name: String, word_instructions: String){
        self.diccionario.insert(word_name,word_instructions);
    }

    pub fn contiene_word(&mut self, word_name: String) -> bool{
        self.diccionario.contains_key(&word_name)
    }

    pub fn obtener_instrucciones(&mut self, word_name: String) -> String{
        let mut instrucciones_en_string: String = String::new();
        match self.diccionario.get(&word_name){
            Some(word_inst) => 
                    instrucciones_en_string.push_str(&word_inst),
            None => (),
        };
        instrucciones_en_string
    }
}