use DocUnit;
use ValDesc;

pub fn print_units(units: Vec<DocUnit>) {
    print_header(format!("Summary"));
    for elem in &units {
        println!(" - {}", elem.fn_name);
    }
    println!();
    for elem in units {
        print_header(format!("Function '{}'", elem.fn_name));
        match elem.description {
            Some(desc) => {
                println!("{}", desc);
                println!();
            },
            None => {}
        };
        println!("=> Arguments");
        println!();
        print_list(elem.arguments);
        println!();
        println!("=> Returns");
        println!();
        print_list(elem.returns);
        println!();
        match elem.throws {
            Some(throws) => {
                println!("=> Throws");
                println!();
                print_list(throws);
                println!();
            },
            None => {}
        }
    }
}

pub fn print_list(list: Vec<ValDesc>) {
    let mut max_len = 0;
    for elem in &list {
        if elem.0.len() > max_len {
            max_len = elem.0.len();
        }
    }
    for elem in list {
        let size = elem.0.len();
        print!(" - {}", elem.0);
        for _ in size..max_len {
            print!(" ");
        }
        println!("    {}", elem.1);
    }
}

pub fn print_header(header: String) {
    println!("{}", header);
    for _ in 0..header.len() {
        print!("=");
    }
    println!("\n");
}