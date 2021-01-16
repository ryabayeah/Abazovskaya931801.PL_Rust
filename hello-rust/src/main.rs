extern crate rand;
use rand::Rng;
use std::char;
use std::thread;
use std::io::{Read, Write};
use std::str::from_utf8;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::env;
use std::io;



fn client(address: String) 
{
    match TcpStream::connect(address) 
    {   
        Ok(mut stream) => 
        {  
            println!("CONNECTION SUCCESSFUL");

            let mut data = [0 as u8; 50];
            let mut rep = [0 as u8; 50];
           
           loop 
           {
               let hash_str = get_hash_str();
               let session_key = get_session_key();

               let next_key = next_session_key(&hash_str, &session_key);

               println!("Your message: ");
               let mut message = String::new();

               io::stdin().read_line(&mut message);

               stream.write(&hash_str.into_bytes()).unwrap();
               stream.write(&session_key.into_bytes()).unwrap();
               stream.write(&message.into_bytes()).unwrap();

               match stream.read(&mut data) {
                   Ok(size) => 
                   {
                       stream.read(&mut rep);
                       let received_key = from_utf8(&data[0..size]).unwrap();
                       let response = from_utf8(&rep).unwrap();

                       if received_key == next_key {println!("CLIENT KEY: {}\nSERVER KEY: {}", next_key, received_key);} 
                       else {break;}

                       println!("RESPONSE: {}", response);
                   }, 
                   Err(e) => {println!("DATA RECIVE FAILED: {}", e);}
               }
           }
        }, 
        Err(e) => {println!("CONNECTION FAILED: {}", e);
        }
    }
    println!("SHUT DOWN CLIENT\n--------------------------------");
}


fn server(port: String,max_clients: u64) 
{   
    let ip = "127.0.0.1:".to_string();
    let session = ip + &port;
    let listener = TcpListener::bind(session.to_string()).unwrap();
    println!("SERVER {} LISTENING...",session);
    let _max = 0;
    for stream in listener.incoming() 
    {
        match stream 
        {
            Ok(stream) => 
            {   let _max = _max + 1;
                println!("{}**", _max);
                if(_max <= max_clients)
                {
                    println!("NEW CONNECTION: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {handle_request(stream)});
                }
                else{println!("EXCEEDED THE NUMBER OF CLIENTS PER SESSION");}
                
            }
            Err(e) => {println!("ERROR: {}", e);}
        }
    }
    drop(listener);
}

fn handle_request(mut stream: TcpStream) {
    let mut hash = [0 as u8; 5];
    let mut key = [0 as u8; 10];
    let mut message = [0 as u8;50];

    while match stream.read(&mut hash) 
    {
        Ok(_) => 
        {
            stream.read(&mut key);
            stream.read(&mut message);

            let received_hash = from_utf8(&hash).unwrap();
            let received_key = from_utf8(&key).unwrap();
            let new_key = next_session_key(&received_hash,&received_key);
            let result = new_key.clone().into_bytes();

            stream.write(&result).unwrap();
            stream.write(&message).unwrap();
            true
        },
        Err(_) => 
        {
            println!("CONNECTION ERROR WITH: {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}


fn get_session_key() -> String 
{
    let mut key = String::new();
    let mut rng = rand::thread_rng();

    for _i in 0..10 
    {
        let num = rng.gen_range(1..10);
        let ch = char::from_digit(num, 10).unwrap();
        key.push(ch);
    }

    return key;
}

fn get_hash_str() -> String 
{
    let mut hash_str = String::new();
    let mut rng = rand::thread_rng();

    for _i in 0..5 
    {
        let num = rng.gen_range(1..7);
        let ch = char::from_digit(num, 10).unwrap();
        hash_str.push(ch);
    }

    return hash_str;
}

fn next_session_key(hash_str: &str, session_key: &str) -> String 
{
    if hash_str.is_empty() 
    {
        return "HASH CODE IS EMPTY".to_string()
    }

    for ch in hash_str.chars() 
    {
        if !ch.is_ascii_digit() 
        {
            return "HASH CODE CONTAINS NON-DIGIT LETTER".to_string()
        }
    }

    let mut result = 0;

    for ch in hash_str.chars() 
    {
        let l = ch.to_string();
        result += calc_hash(session_key.to_string(), l.parse::<u64>().unwrap()).parse::<u64>().unwrap();
    }

    return result.to_string();
}

fn calc_hash(key: String, value: u64) -> String 
{
    match value
    {
        1=>{
            let chp = "00".to_string() + &(key[0..5].parse::<u64>().unwrap() % 97).to_string();
            return chp[chp.len() - 2..chp.len()].to_string()
            }

        2=>{
            let reverse_key = key.chars().rev().collect::<String>();
            return reverse_key + &key.chars().nth(0).unwrap().to_string()
            }

        3=>{
            return key[key.len() - 5..key.len()].to_string() + &key[0..5].to_string()
            }

        4=>{
            let mut num = 0;
            for _i in 1..9 
            {
                num += key.chars().nth(_i).unwrap().to_digit(10).unwrap() as u64 + 41;
            }
            return num.to_string()
            }

        5=>{
            let mut ch: char;
            let mut num = 0;
    
            for _i in 0..key.len() 
            {
                ch = ((key.chars().nth(_i).unwrap() as u8) ^ 43) as char;
                if !ch.is_ascii_digit() 
                {
                    ch = (ch as u8) as char;
                }
                num += ch as u64;
            }
            return num.to_string()
            }

        _=>{
            return (key.parse::<u64>().unwrap() + value).to_string()
            }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let address = &args[1];
    let max_clients = &args[3].to_string();
    
    if ( args[1].len() > 5 ) 
    {
         client(address.to_string());
    } 
    else 
    {   
        server(address.to_string(),max_clients.parse::<u64>().unwrap()); 
    }
    
}
