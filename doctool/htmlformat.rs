use DocUnit;
use ValDesc;

pub fn print_units(units: Vec<DocUnit>) {
    let mut style = String::new();
    style.push_str("td {");
    style.push_str("padding-left: 20px");
    style.push_str("};");
    println!("<html>\n<head>\n<style>{}</style></head>\n<body>", style);
    println!("<h2>Summary</h2>");
    println!("<ul>");
    for elem in &units {
        println!("<li>{}</li>", elem.fn_name);
    }
    println!("</ul><hl>");
    
    println!("<h1>Functions</h1>");
    for elem in units {
        println!("<h2>Function '{}'</h2>", elem.fn_name);
        match elem.description {
            Some(desc) => {
                println!("<div>{}</div>", desc);
            },
            None => {}
        };
        println!("<h3>Arguments</h3>");
        print_list(elem.arguments);
        println!("<h3>Returns</h3>");
        print_list(elem.returns);
        match elem.throws {
            Some(throws) => {
                println!("<h3>Throws</h3>");
                print_list(throws);
            },
            None => {}
        }
    }
    println!("</body>");
}

pub fn print_list(list: Vec<ValDesc>) {
    println!("<table>");
    for elem in list {
        println!("<tr>");
        println!("<td>{}</td>", elem.0);
        println!("<td>{}</td>", elem.1);
        println!("</tr>");
    }
    println!("</table>");
}