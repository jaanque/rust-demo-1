use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

// Esta es la función que se ejecutará cuando alguien visite la ruta raíz "/"
async fn root() -> &'static str {
    "¡Hola desde nuestro BaaS con Rust!"
}

// La función `main` es el punto de entrada de nuestra aplicación
#[tokio::main]
async fn main() {
    // 1. Definimos las rutas de nuestra aplicación.
    // Por ahora, solo tenemos una ruta: la raíz "/" que responde con el método GET.
    // Cuando se reciba una petición GET a "/", se llamará a nuestra función `root`.
    let app = Router::new().route("/", get(root));

    // 2. Definimos la dirección y el puerto donde nuestro servidor escuchará.
    // "0.0.0.0" significa que escuchará en todas las interfaces de red disponibles.
    // El puerto 3000 es una elección común para desarrollo.
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Servidor escuchando en http://{}", addr);

    // 3. Creamos el listener (oyente) de TCP.
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // 4. Iniciamos el servidor para que empiece a aceptar conexiones.
    axum::serve(listener, app).await.unwrap();
}