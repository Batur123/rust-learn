use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

#[derive(Debug)]
struct Request {
    method: String,
}

// Assuming a complex data structure as described
struct ComplexData {
    somefield: Vec<i32>,
    another: AnotherField,
}

struct AnotherField {
    nested_arr: Vec<char>,
    some: SomeField,
}

struct SomeField {
    number: i32,
}

impl ComplexData {
    // Convert to a manually formatted JSON string
    fn to_json(&self) -> String {
        let somefield_str = self.somefield.iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let nested_arr_str = self.another.nested_arr.iter()
            .map(|ch| format!(r#""{}""#, ch))
            .collect::<Vec<String>>()
            .join(",");

        let json = format!(
            r#"{{
                "somefield": [{}],
                "another": {{
                    "nested_arr": [{}],
                    "some": {{
                        "number": {}
                    }}
                }}
            }}"#,
            somefield_str,
            nested_arr_str,
            self.another.some.number
        );

        json
    }
}

fn handle_request(http_request: &Vec<String>, mut stream: TcpStream) {
    // Example data construction
    let complex_data = ComplexData {
        somefield: vec![1, 2, 3, 4, 5],
        another: AnotherField {
            nested_arr: vec!['a', 'b'],
            some: SomeField { number: 1 },
        },
    };

    // Convert the complex structure to JSON string
    let json_response = complex_data.to_json();

    // Construct the HTTP response with JSON content type
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
        json_response
    );

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
    handle_request(&http_request, stream);
}
