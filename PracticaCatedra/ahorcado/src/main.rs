mod file_manager;
mod ahorcado;

fn main() {
    let palabras = file_manager::lectura_archivo("palabras.txt");
    ahorcado::inicio_ahorcado(palabras);
}
