mod estructuras;
mod flujos;
mod variables;

fn main() {
    print!("¡Hola! ");
    print!("He venido a ");
    println!("decir...");
    println!("HOLA MUNDO");

    // Variables y tipos de datos
    variables::pruebas();

    // Estructuras de datos
    estructuras::listas();
    estructuras::sets();
    estructuras::mapas();

    // Control de flujos y bucles
    flujos::pruebas();

}
