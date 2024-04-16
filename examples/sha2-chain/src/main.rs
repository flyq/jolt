pub fn main() {
    let (prove_sha2_chain, verify_sha2_chain) = guest::build_sha2_chain();

    let input = [5u8; 32];
    let iters = 10;
    let (output, proof) = prove_sha2_chain(input, iters);
    proof.save_to_file("./sha2_chain10.bin").unwrap();
    let is_valid = verify_sha2_chain(proof);

    println!("output: {}", hex::encode(output));
    println!("valid: {}", is_valid);
}
