use common::{Input, Output};
use methods::{RANGE_PROOF_ELF, RANGE_PROOF_ID};
use num_bigint::BigUint;
use risc0_zkvm::{
    get_prover_server, ExecutorEnv, ExecutorImpl, ProveInfo, ProverOpts, Receipt, VerifierContext,
};
use std::time::Instant;

fn main() {
    let input = setup_inputs();

    let env = setup_env(&input.base, &input.modulus, &input.range);

    // Generate proof and get receipt
    let receipt = generate_proof(env).receipt;

    let output: Output = receipt.journal.decode().unwrap();

    println!("u: {}, range: {}", output.u, output.range);
    // Verify the proof
    verify_proof(&receipt);
}

fn setup_inputs() -> Input {
    let input = Input::new_default();

    Input {
        base: input.base,
        modulus: input.modulus,
        range: input.range,
    }
}

fn setup_env<'a>(base: &'a BigUint, modulus: &'a BigUint, range: &'a BigUint) -> ExecutorEnv<'a> {
    let input = Input {
        base: base.clone(),
        modulus: modulus.clone(),
        range: range.clone(),
    };

    ExecutorEnv::builder()
        .write_slice(&bincode::serialize(&input).unwrap())
        .build()
        .unwrap()
}

fn generate_proof(env: ExecutorEnv) -> ProveInfo {
    let mut exec = ExecutorImpl::from_elf(env, RANGE_PROOF_ELF).unwrap();
    let exec_start = Instant::now();
    let session = exec.run().unwrap();
    let exec_duration = exec_start.elapsed();
    println!("Session execution completed in {:?}", exec_duration);

    // Recursive proving, fast verification
    // let prover = get_prover_server(&ProverOpts::succinct()).unwrap();
    // Fast proving, slow verification
    let prover = get_prover_server(&ProverOpts::fast()).unwrap();
    let ctx = VerifierContext::default();

    println!("Starting proof generation...");
    let proof_start = Instant::now();
    let prove_info = prover.prove_session(&ctx, &session).unwrap();
    let proof_duration = proof_start.elapsed();
    println!("Proof generation completed in {:?}", proof_duration);
    prove_info
}

fn verify_proof(receipt: &Receipt) {
    println!("Starting verification...");
    let verify_start = Instant::now();
    receipt.verify(RANGE_PROOF_ID).unwrap();
    let verify_duration = verify_start.elapsed();
    println!("Verification completed in {:?}", verify_duration);

    println!("Verified");
}
