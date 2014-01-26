use std::os;
use std::io::buffered::BufferedReader;
use std::io::File;
use std::str;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile> <inputfile>", args[0]); 
    } else {
        let fname1 = args[1].clone();
        let fname2 = args[2].clone();
        let path1 = Path::new(fname1.clone());
        let path2 = Path::new(fname2.clone());
        let file1 = File::open(&path1);
        let file2 = File::open(&path2);

	let mut message: ~[u8];
	let x;
	let y;
	
        match (file1) {
            Some(mut msg1) => {
                let msg1_bytes: ~[u8] = msg1.read_to_end();
                x = msg1_bytes;
            } ,
            None => fail!("Error opening message files: {:s}", fname1)
        }
        
        match (file2) {
            Some(mut msg2) => {
                let msg2_bytes: ~[u8] = msg2.read_to_end();
                y = msg2_bytes;
            } ,
            None => fail!("Error opening message files: {:s}", fname2)
        }
        message = xor(x, y);
        println!("Decoded messsage: {:s}", str::from_utf8(message));
    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}