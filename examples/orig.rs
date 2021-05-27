use ripemd160::{Digest, Ripemd160};
use std::env;
use std::fs;
use std::io::{self, Read};

const BUFFER_SIZE: usize = 1024;

/// Print digest result as hex string and name pair
fn print_result(sum: &[u8], name: &str) {
    for byte in sum {
        print!("{:02x}", byte);
    }
    println!("\t{}", name);
}

/// Compute digest value for given `Reader` and print it
/// On any error simply return without doing anything
fn process<D: Digest + Default, R: Read>(reader: &mut R, name: &str) {
    let mut sh = D::new();
    let mut buffer = [0u8; BUFFER_SIZE];
    loop {
        let n = match reader.read(&mut buffer) {
            Ok(n) => n,
            Err(_) => return,
        };
        println!("bytes: {:?}", n);
        println!("bytes: {:?}", &buffer[..n]);
        sh.update(&buffer[..n]);
        if n == 0 || n < BUFFER_SIZE {
            break;
        }
    }
    print_result(&sh.finalize(), name);
}

fn main() {
    let args = env::args();
    // Process files listed in command line arguments one by one
    // If no files provided process input from stdin
    if args.len() > 1 {
        for path in args.skip(1) {
            if let Ok(mut file) = fs::File::open(&path) {
                process::<Ripemd160, _>(&mut file, &path);
            }
        }
    } else {
        process::<Ripemd160, _>(&mut io::stdin(), "-");
    }
}

/*
<-- "correct horse battery staple" 문자열에 대해서,


* https://md5calc.com/hash/ripemd160/correct+horse+battery+staple
    5e708aa85ae8b0d080837c50bd63634d584edc00

* https://www.online-convert.com/

    hex: 5e708aa85ae8b0d080837c50bd63634d584edc00
    HEX: 5E708AA85AE8B0D080837C50BD63634D584EDC00
    h:e:x: 5e:70:8a:a8:5a:e8:b0:d0:80:83:7c:50:bd:63:63:4d:58:4e:dc:00
    base64: XnCKqFrosNCAg3xQvWNjTVhO3AA=


----

그런데,

$ echo "correct horse battery staple" | cargo run --example ripemd160sum
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/examples/ripemd160sum`
5811d8a783db6eb4457dcaf42a0b705b56462111	-

$ echo correct horse battery staple | cargo run --example ripemd160sum
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/examples/ripemd160sum`
5811d8a783db6eb4457dcaf42a0b705b56462111


...왜 다르지? -,.- [Thu May 27 06:10:53 PM KST 2021]


*/
