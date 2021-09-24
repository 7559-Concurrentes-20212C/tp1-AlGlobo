use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::thread;

/* (archivo con de configuracion con rate limit y numero de aerolineas)
/ 1. inicializar web services de aerolineas y hotel (struct)
/ 2. leer pedidos del archivo y spawnear 1 thread (thread cliente) por pedido
/ 3. cada thread cliente invoca el metodo "procesar" del struct aerolinea y se queda esperando a que este retorne
/ 4. el metodo procesar, spawnea internamente un thread que simulara el trabajo a realizar
        controlar rate limit
/ 5. el cliente recibe el valor de retorno del metodo procesar y finaliza
*/

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file = File::open(filename).expect("could not open file");
    let reader = BufReader::new(file);
    let mut threads = vec![];

    for line in reader.lines().flatten() {
        threads.push(thread::spawn(move || process(line) ));

    }

    for thread in threads {
        thread
            .join()
            .expect("Couldn't join on the associated thread");
    }
}

fn process(text: String) {
    println!("{}", text);
}
