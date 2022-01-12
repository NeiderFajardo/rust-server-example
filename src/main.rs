use server::Server;

// Rust trata los archivos como módulos, por lo que es importante usar el mismo
// nombre del archivo para poder importar módulos externos
mod server;
mod http;

fn main() {
    //let get = Method::GET;
    //let delete = Method::DELETE;
    //let post = Method::POST;
    //let put = Method::PUT;
    //let string_prueba = String::from("127.0.0.1:8080");
    //let string_slice = &string_prueba[10..];
    // Si no se pone una referencia a esta línea el compilador muestra un error ya que
    // la variable ha sido presatada al string_slice.
    //dbg!(&string_prueba);
    //dbg!(string_slice);

    // En Rust hay dos tipos de Strins, este es un String slice(&str) referencia inmutable
    // a una parte de un string.
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}


