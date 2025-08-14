#[derive(Debug)]
struct Product {
    name: String,
    price: f32,
    stock: bool
}

impl Product {
    fn new(name:String, price:f32) -> Self{ //los nombres de argumentos y keys son los mismos
        Self{
            name, //Por eso se ponen una sola vez.
            price,
            stock: true
        }
    }

    fn con_iva(&self)-> f32{
        self.price * 1.21
    }

    fn sell(&mut self) {
        self.stock = false
    }
}

fn main() {

    let mut laptop= Product::new("Mac M4".to_string(), 1200.0);

    println!("Producto: {} \nPrecio: {} \n{}", laptop.name, laptop.price, if laptop.stock{"En Stock"}else{"Sin Stock"});

    println!("Precio con IVA: {}", laptop.con_iva());

    laptop.sell();

    println!("Producto: {} \nPrecio: {} \n{}", laptop.name, laptop.price, if laptop.stock{"En Stock"}else{"Sin Stock"});

}
