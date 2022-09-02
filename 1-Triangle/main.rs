use std::io;

fn main() {

    let mut input = String::new();
    println!("Triangle levels?: " );
    match io::stdin().read_line(&mut input) {   
        Ok(_) =>{
            let inp = input.trim_end().parse::<u32>();
            match inp{
                Ok(val) => println!("{}",render_triangle(val)),
                Err(e) => println!("Wrong input format: {}",e),
            }
        },
        Err(e) => println!("Something went wrong: {}",e),
    }
}


fn render_triangle(n:u32) -> String{
    let mut triangle: String = String::from("");
    let ast = "*";
    for i in 1..n + 1{
        let row = ast.repeat(i as usize);
        triangle.push_str(&"\n".to_string());
        triangle.push_str(&row); 
}
    triangle
}

