pub fn pruebas() {
    println!("Pruebas con variable y constantes");

    /*
     * Variables - inmutables por defecto
     */
    let my_string = "Soy un string bonito"; //Inferencia
    imprimir_string(my_string);
    let my_string2: String = String::from("String difiere de &str"); // &str es distinto de String, aunque no sé en qué difieren
    println!("Otro tipo de cadena: {my_string2}");

    let my_int: i32 = 32; //tipado explícito, sin inferencia
    imprimir_int(my_int as i128);  // TIPADO FUERTE - Casteo la variable 'i32' a 'i128'
    imprimir_int(my_int.into());   // Otra forma de pasarle el 'i32' a la función ????

    let my_float: f32 = 3.14;
    imprimir_float(my_float as f64);

    /*
     * Variables mutables
     */ 
    let mut my_mut_int: i64 = 64;
    imprimir_int(my_mut_int as i128);

    // TIPADO FUERTE - ERROR - NO PUEDO MEZCLAR TIPOS DE DATOS
    //my_mut_int = my_mut_int + my_int;
    my_mut_int = my_mut_int + my_int as i64; // Necesito castear el 'i32' para poder sumarlo al 'i64'
    imprimir_int(my_mut_int as i128);

    /*
     * Constantes - No se pueden modificar de ninguna forma
     */
    // Las constantes se tienen que escribir con mayúsculas
    const MY_CONST: i64 = 23; // El tipo de dato de las constantes no se puede inferir
    imprimir_int(MY_CONST as i128); // Puedo castear la constante

    println!();
}

/*
 * Función privada
 */
fn imprimir_string(my_var: &str) {
    //println!("No puedo terminar de creer que haya funciones privadas. ¿Qué diablos es este archivo?")
    println!("La cadena '{}' vale '{}'", stringify!(my_var), my_var)
}

fn imprimir_int(my_var: i128) {
    //println!("No puedo terminar de creer que haya funciones privadas. ¿Qué diablos es este archivo?")
    println!("El entero '{}' vale '{}'", stringify!(my_var), my_var)
}

fn imprimir_float(my_var: f64) {
    //println!("No puedo terminar de creer que haya funciones privadas. ¿Qué diablos es este archivo?")
    println!("El flotante '{}' vale '{}'", stringify!(my_var), my_var)
}

