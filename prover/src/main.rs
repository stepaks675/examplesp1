use sp1_sdk::{include_elf, ProverClient, SP1Stdin, HashableKey};
use sp1_verifier::{Groth16Verifier,  GROTH16_VK_BYTES};
/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_elf!("example");

#[derive(Debug)]
struct ProofData {
    proof: String,         
    public_inputs: String, 
    vkey_hash: String,     
}

fn main() {
    // our initial string, 
    let input_string = "Hello, Succinct!".to_string();

    // create an input stream and write the string to it.
    let mut stdin = SP1Stdin::new();
    stdin.write(&input_string);

    // create a `ProverClient` method.
    let client = ProverClient::from_env();
	
	println!("starting proving");
	
    // generate the proof for the given program and input.
    let (pk, vk) = client.setup(ELF);
    let proof = client.prove(&pk, &stdin).groth16().run().expect("proof generation failed");

    println!("generated proof");
	
	let fixture = ProofData {
        proof: hex::encode(proof.bytes()),
        public_inputs: hex::encode(proof.public_values),
        vkey_hash: vk.bytes32(),
    };
	


	println!("{:?}", fixture);
	
	let proof_bytes = hex::decode(fixture.proof).expect("Invalid hex in proof");
	let public_inputs_bytes = hex::decode(fixture.public_inputs).expect("Invalid hex in public inputs");
	let vkey_hash_str = fixture.vkey_hash.as_str(); // Получаем `&str` из `String`

	let result = Groth16Verifier::verify(&proof_bytes, &public_inputs_bytes, vkey_hash_str, *GROTH16_VK_BYTES).is_ok();
	println!("{:?}", result);
}