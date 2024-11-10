use std::rc::Rc;

fn main() {
    let valor = "Un valor".to_string();

    let r_valor = Rc::new(valor);

    let r_valor2 = Rc::clone(&r_valor);
    let r_valor3 = r_valor.clone();

    println!("\n=================================================\n");
    println!(
        "¿Los valores de los apuntadores son los mismos? \n¿ {:p} == {:p} ? {}",
        r_valor2.as_ptr(),
        r_valor3.as_ptr(),
        std::ptr::eq(r_valor2.as_ptr(), r_valor3.as_ptr())
    );

    println!("\n=================================================\n");

    // weak apunta al Rc, no implementa Deref
    let weak_1 = Rc::downgrade(&r_valor);
    let weak_2 = Rc::downgrade(&r_valor2);

    println!("¿Sigue vivo? {:p}", r_valor.as_ptr());

    println!("\n=================================================\n");
    println!("¿Weak apunta a lo mismo? {:p}", weak_1.as_ptr());
    println!(
        "¿Dos weaks apuntan a lo mismo? {:p} == {:p}",
        weak_1.as_ptr(),
        weak_2.as_ptr()
    );

    //drop(r_valor);
    //drop(r_valor2);
    //drop(r_valor3);
    println!("\n=================================================\n");
    unsafe {
        println!("¿Qué contiene el weak? {:?}", *weak_1.into_raw());
    }
}
