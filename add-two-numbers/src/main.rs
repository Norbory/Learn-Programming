struct LinkedList {
    value: char,
    next: Option<Box<LinkedList>>,
}

impl LinkedList {
    // Constructor que toma un valor y se inicializa
    fn new(value: char) -> Self {
        Self {
            value,
            next: None,
        }
    }
    
    // Método para agregar un nodo al final de la lista
    fn append(&mut self, value: char) {
        match &mut self.next {
            None => {
                self.next = Some(Box::new(LinkedList::new(value)));
            },
            Some(next_node) => {
                next_node.append(value);
            }
        }
    }
    
    // Método para imprimir la lista
    fn print(&self) {
        print!("{}", self.value);
        if let Some(next) = &self.next {
            print!(" -> ");
            next.print();
        } else {
            println!(" -> Fin de la lista");
        }
    }
}

fn main() {
    // Crear una lista enlazada usando el constructor
    let mut lista = LinkedList::new('A');
    
    // Agregar más elementos usando el método append
    lista.append('B');
    lista.append('C');
    lista.append('D');
    
    // Imprimir la lista usando el método print
    println!("Lista completa:");
    lista.print();
    
    // Ejemplo alternativo: crear manualmente
    println!("\nEjemplo alternativo:");
    let mut manual_list = LinkedList::new('X');
    manual_list.next = Some(Box::new(LinkedList::new('Y')));
    if let Some(ref mut next) = manual_list.next {
        next.next = Some(Box::new(LinkedList::new('Z')));
    }
    manual_list.print();
}

// Implementación de una lista enlazada simple en Rust