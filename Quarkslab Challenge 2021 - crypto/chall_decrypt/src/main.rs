use std::env;
use std::process;
use std::vec::Vec;

use std::fs::File;
use std::io::prelude::*;
use std::io::Read;


extern crate quadratic_congruence_solver;
mod aes_bindings;

fn help() {
    println!("usage: ");
    println!("chall_decrypt <inputfile> <outputfile>");
}

fn process_args() -> ( String, String) {
    let args: Vec<String> = env::args().collect();
    let filename;
    let outfilename;


    match args.len() {
    3 => {
            filename = args[1].clone();
            outfilename = args[2].clone();
         },
    _ => {
             help();
             process::exit(1);
         }
    }

    (filename, outfilename)
}

fn main() {

    let (filename, outfilename) = process_args();

    let mut aes_ctx = 
        aes_bindings::AES_ctx{RoundKey : [0; aes_bindings::AES_KEYEXPSIZE], Iv: [0; aes_bindings::AES_BLOCKLEN]};
    let key: [u8; aes_bindings::AES_KEYLEN] = 
        [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
    aes_bindings::init_aes(&mut aes_ctx, &key);
    aes_ctx.RoundKey = [0; aes_bindings::AES_KEYEXPSIZE];  // zero expanded key in aes context


    let xor_constant: [u8; aes_bindings::AES_BLOCKLEN] = 
        [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10];
    let mut round_iv: [u8; aes_bindings::AES_BLOCKLEN] = 
        [0x8f, 0x10, 0x4d, 0x30, 0x86, 0x56, 0x29, 0xf2, 0xd7, 0x5f, 0x93, 0x69, 0xe5, 0x1a, 0xfa, 0x00];

    let mut f = File::open(&filename).expect("no input file found");
    let mut outfile = File::create(&outfilename).expect("coud not create oputput file");

    let mut first_file_block  = [0 ; aes_bindings::AES_BLOCKLEN];
    let mut second_file_block  = [0 ; aes_bindings::AES_BLOCKLEN];

    f.read_exact(&mut first_file_block).expect("file to short");
    let mut first_adjecent_block_round11 = solve_quadratics(&mut first_file_block);

    loop {

        match f.read_exact(&mut second_file_block) {
            Err(_v) => break,
            Ok(_v) =>{}
        }

        let second_adjecent_block_round11  = solve_quadratics(&mut second_file_block);

        let mut decrypt_array: [u8;16] = 
            xor_blocks(first_adjecent_block_round11, second_adjecent_block_round11);

        aes_bindings::decrypt_aes_ecb(&mut aes_ctx, &mut decrypt_array);

        let xor_buf_1: [u8; aes_bindings::AES_BLOCKLEN] = xor_blocks(decrypt_array, xor_constant);
        let xor_buf_2: [u8; aes_bindings::AES_BLOCKLEN] = xor_blocks(round_iv, xor_buf_1);

        round_iv = xor_buf_2;

        let plain_text_block: [u8; aes_bindings::AES_BLOCKLEN] = xor_blocks(xor_buf_2, first_file_block);


        outfile.write_all(&plain_text_block).expect("outfile write error");


        first_adjecent_block_round11 = second_adjecent_block_round11;
        first_file_block = second_file_block;
    }

}

fn xor_blocks(b1 : [u8; aes_bindings::AES_BLOCKLEN], b2 : [u8; aes_bindings::AES_BLOCKLEN] ) -> [u8; aes_bindings::AES_BLOCKLEN] {
    let c: Vec<u8> = b1.iter().zip(&b2).map(|(x, y)| x ^ y).collect();

    let mut out_array: [u8; aes_bindings::AES_BLOCKLEN] = [0;16];
    out_array.copy_from_slice(&c); 

    out_array
}

fn solve_quadratics(buffer : &[u8 ; 16]) -> [u8 ; 16] {

    let dword1 = u32::from_le_bytes( [buffer[0], buffer[1], buffer[2], buffer[3]] );
    let dword2 = u32::from_le_bytes( [buffer[4], buffer[5], buffer[6], buffer[7]] );
    let dword3 = u32::from_le_bytes( [buffer[8], buffer[9], buffer[10], buffer[11]] );
    let dword4 = u32::from_le_bytes( [buffer[12], buffer[13], buffer[14], buffer[15]] );

    let out1 = solve_q_conqruence(dword1).expect("No solution for quadratic conqruence");
    let out2 = solve_q_conqruence(dword2).expect("No solution for quadratic conqruence");
    let out3 = solve_q_conqruence(dword3).expect("No solution for quadratic conqruence");
    let out4 = solve_q_conqruence(dword4).expect("No solution for quadratic conqruence");

    let mut vec: Vec<u8> = Vec::new();
    vec.extend(out1.to_le_bytes().iter().copied());
    vec.extend(out2.to_le_bytes().iter().copied());
    vec.extend(out3.to_le_bytes().iter().copied());
    vec.extend(out4.to_le_bytes().iter().copied());

    let mut out_array : [u8;16] = [0;16];
    out_array.copy_from_slice(&vec);

    out_array

}

fn solve_q_conqruence(arg : u32) -> Option<u32> {
    let mut params = quadratic_congruence_solver::Coefs { 
        a : 621221138,
        b : -1427868995,
        c : 211712510,
        d : arg as i64,
        n : 4294967296
    };

    let sol = quadratic_congruence_solver::solver::solve_quadratic(&mut params);

    let mut sol_exists = true;
    for s in &sol {
        if *s == -1 {
            sol_exists = false;
            break;
        }
    }

    if !sol_exists {
        None
    }else {
        Some(sol[0] as u32)
    }
}
