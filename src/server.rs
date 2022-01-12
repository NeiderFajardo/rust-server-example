use std::io::Read;
use crate::http::Request;
use std::net::TcpListener;
use std::convert::TryFrom;
// Así es como se define en Rust un a estructura con sus atributos
pub struct Server {
    addr: String,
}
// Aquí es donde definimos los métdos de la estructura
// Hay dos tipos 'funciones', los métodos y las funciones asociadas, las cuales son similares a
// una función estática.
impl Server {
    // Esto es una función asociada, no necesita una implementación previa
    // Se puede intercambiar la palabra Server por Self
    pub fn new(addr: String) -> Server {
        // Cuando no se usa explicitamente la palabra return, se retorna lo que haya en la
        // última línea del método/función
        Server {
            // Ya que el parámetro que recibímos en la función tiene el mismo nombre
            // que el parámetro de la estructura se puede dejar así 
            addr
        }
    }
    // Esto es un método, se pasa una referencia de la propua estructura, hay que tener cuidad
    // con las reglas de propiedad, ya que si se deja así, el método será el propietario de la
    // estructura y cuando se termine de ejecutar se desasignará la memoria de la estructura
    // Para evitar esto se puede pasar una referencia(prestamo). En este caso no importa
    // porque la idea es que sea un loop infinito.
    pub fn run(self) {
        print!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        // Esta es una característica especial de Rust, representa un bucle infinito
        // como hacer un while true (que también es válido)
        loop {
            // Esto es parecido a un switch, es necesario validar todas las expresiones
            // que retoran,o enums. Se pueden ignorar variables con _ y también 
            // crear un caso por default.
            match listener.accept() {
                // Estas dos expresiones vienen porque la respuesta del .accept esta encapsulado
                // en un Result.. y estas son las dos respuestas.
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    // Ejemplo de un match con más de una línea
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            print!("Recieved a request: {}", String::from_utf8_lossy(&buf));

                            Request::try_from(&buf as &[u8]);
                            // La anterior línea también se puede pasar de la siguiente forma
                            //convirtiendo la variable en un arreglo genérico
                            // Request::try_from(&buf[..])

                            /*
                            Ya que Rust ofrece la implementación opuesta del try_from sin tener
                            que definirla expresamente en código, podemos llamarla de la siguiente forma.
                            Es importante recordar que debemos usar la libreria TryInto
                            let res: &Result<Request, _> = &buf[..].try_into();
                            */
                        }
                        Err(e) => println!("Failed to establish a connection: {}", e),
                    }
                },
                Err(e) => print!("Failed to establish a connection {}", e),
            }
        }
    }
}  