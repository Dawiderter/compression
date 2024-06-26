use arithmetic::{coder::Coder, decoder::Decoder};

fn main() {
    let input_stream = include_str!("../../testy/pride_and_prejudice.txt").as_bytes();
    let input_len = input_stream.len();
    let mut output_stream = Vec::<u8>::new();
    
    let mut coder = Coder::new(input_stream, &mut output_stream);
    
    coder.code_all();
    let comp_len = output_stream.len();
    
    //dbg!(&output_stream);
    
    let mut decode_output_stream = Vec::<u8>::new();
    
    let mut decoder = Decoder::new(output_stream.as_slice(), &mut decode_output_stream);
    
    decoder.decode_all();
    
    println!("{}", String::from_utf8(decode_output_stream).unwrap());
    dbg!(input_len,comp_len, input_len as f32 / comp_len as f32);
}
