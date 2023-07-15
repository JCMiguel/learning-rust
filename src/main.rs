mod estructuras;
mod flujos;
mod variables;

fn main() {
    print!("¡Hola! ");
    print!("He venido a ");
    println!("decir...");
    println!("HOLA MUNDO"); // Creo que estas "funciones" que llevan '!' en realidad son macros.

    // Variables y tipos de datos
    variables::pruebas();
    //variables::imprimir_int(32); // No tengo acceso desde acá a esta función

    // Estructuras de datos
    estructuras::listas();
    estructuras::sets();
    estructuras::mapas();

    // Control de flujos y bucles
    flujos::pruebas();

}
