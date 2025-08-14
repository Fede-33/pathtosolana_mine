struct Rectangle {
    base: u32,
    altura: u32
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.base * self.altura
    }

    fn perimetro (&self) -> u32 {
        (self.base +self.altura)*2
    }
}

impl Rectangle { // Puede haber dos bloques de implementación sobre el mismo struct
    fn cuadrado (lado:u32)-> Self {
        Self {
            base: lado,
            altura: lado
        }
    }
    fn position (&self) -> String {
        (if self.base > self.altura{
            "Horizontal"
        } else if self.base < self.altura{
            "Vertical"
        } else {
            "Cuadrado"
        }).to_string()
        
    }
}


fn main() {
    
    let rectangle1 = Rectangle { base: 10, altura:15};
    println!("Área: {}", rectangle1.area());
    println!("Perímetro: {}", rectangle1.perimetro());
    println!("Posición: {}", rectangle1.position());

    let cuadrado1 = Rectangle::cuadrado(20);
    println!("Área: {}", cuadrado1.area());
    println!("Perímetro: {}", cuadrado1.perimetro());
    println!("Posición: {}", cuadrado1.position());
}
