use std::collections::HashMap;
use std::io;

fn mean(numbers: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for n in numbers.iter() {
        sum = sum + n
    }
    let avg = sum as f64 / numbers.len() as f64;
    avg
}


fn median(numbers: &Vec<i32>) -> f64 {
    let mut sorted = numbers.clone();
    sorted.sort();
    let first = sorted[0];
    let last = sorted[sorted.len() - 1];
    let median = first as f64 + last as f64 / 2 as f64;
    median
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for i in numbers {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    
    let mut biggest = 0;
    let mut big_key = 0;
    for (key, value ) in map {
        if value > biggest {
            biggest = value;
            big_key = *key;
        }
    }
    big_key
}

fn main() {
    {
        let numbers: Vec<i32> = vec![0,2,3,4,8,97,1,26,8,64,21,3,50,50,50];
        
        println!("---------------Vectors------------");
        println!("Mean: {}", mean(&numbers));
        println!("Median: {}", median(&numbers));
        println!("Mode: {}", mode(&numbers));
    }
    {
        println!("---------------Commands------------");
          let mut departaments = HashMap::new();
        loop {
          
            let mut command = String::new();
        
            println!("Digit the command: Add <Person> to <Department> ");
            
            io::stdin().read_line(&mut command).expect("Erro ao ler variável");
            let command = command.trim();
            let mut token_command = command.split_whitespace();
            
            let tem_p =  token_command.nth(1);
            let tem_d =  token_command.nth(1);

            let person = match tem_p {
                Some(p) => p,
                None => {
                    println!("1. Command not found");
                    continue;
                }
            };

            let departament = match tem_d {
                Some(d) => d,
                None => {
                    println!("2. Command not found");
                    continue;
                }
            };

            println!("Person: {:?}", person);
            println!("Department: {:?}", departament);

            let empregados = departaments.entry(String::from(departament)).or_insert(vec![]);
            empregados.push(String::from(person));
            println!("{:?} \n\n\n", departaments)
        }
        

    }
    
    
}
