use std::io;
mod ascii_menu;

fn main() {
    while true{
        let mut option_string=String::new();
        ascii_menu::menu();
        println!("comando:");
        io::stdin().read_line(&mut option_string).expect("erro");
         let option=convert_to_int(&option_string);
        match option{
            1=>sum(),
            2=>sub(),
            3=>div(),
            4=>mult(),
            0=>break,
            _=>println!("opção invalida"),
          }
    }

    }
//converter para inteiro
fn convert_to_int(input: & String)->i32 {
   //trim retira espaços
    let result = input.trim().parse::<i32>().unwrap();
    return result;
}

fn convert_to_real(input: & String)->u32 {
    let result = input.trim().parse::<u32>().unwrap();
    return result;
}
fn convert_to_float(input: & String)->f32 {
    let result = input.trim().parse::<f32>().unwrap();
    return result;
}
fn sub(){

    let mut va = String::new();
    let mut vb = String::new();
  
   
    println!("digite o primeiro valor:");
    io::stdin().read_line(&mut va).expect("erro");

    println!("digite o segundo valor:");
    io::stdin().read_line(&mut vb).expect("erro");

    let v1=convert_to_real(&va);
    let v2=convert_to_real(&vb);

    println!{"{}-{}={}",v1,v2,(v1-v2)}
}
fn mult(){

    let mut va = String::new();
    let mut vb = String::new();
  
   
    println!("digite o primeiro valor:");
    io::stdin().read_line(&mut va).expect("erro");

    println!("digite o segundo valor:");
    io::stdin().read_line(&mut vb).expect("erro");

    let v1=convert_to_real(&va);
    let v2=convert_to_real(&vb);

    println!{"{}*{}={}",v1,v2,(v1*v2)}
}
fn div(){

    let mut va = String::new();
    let mut vb = String::new();
  
   
    println!("digite o primeiro valor:");
    io::stdin().read_line(&mut va).expect("erro");

    println!("digite o segundo valor:");
    io::stdin().read_line(&mut vb).expect("erro");

    let v1=convert_to_float(&va);
    let v2=convert_to_float(&vb);

    println!{"{}/{}={}",v1,v2,(v1/v2)}
}
fn sum(){
    let mut va = String::new();
    let mut vb = String::new();
  
   
    println!("digite o primeiro valor:");
    io::stdin().read_line(&mut va).expect("erro");

    println!("digite o segundo valor:");
    io::stdin().read_line(&mut vb).expect("erro");

    let v1=convert_to_int(&va);
    let v2=convert_to_int(&vb);

    println!{"{}+{}={}",v1,v2,(v1+v2)}
   }
