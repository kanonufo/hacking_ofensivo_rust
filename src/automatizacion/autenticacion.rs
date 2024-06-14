fn verificar_credenciales(usuario: &str, contraseña: &str) -> Result<bool, String> {
    // Simulamos la verificación de credenciales
    if usuario == "admin" && contraseña == "password123" {
        Ok(true)
    } else if usuario == "admin" {
        Err("Contraseña incorrecta".to_string())
    } else {
        Err("Usuario no encontrado".to_string())
    }
}
fn main() {
    let usuario = "admin";
    let contraseña = "password123";

    match verificar_credenciales(usuario, contraseña) {
        Ok(true) => println!("Acceso concedido."),
        Ok(false) => println!("Acceso denegado."),
        Err(e) => println!("Error de autenticación: {}", e),
    }
}
