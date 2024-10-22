use ark_ec::pairing::Pairing;
use ark_mnt6_298::{constraints, Fq, G1Projective, G2Projective};
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::ConstraintSystem;
use ark_std::UniformRand;

use constraints::{G1Var, G2Var};

fn main() {
    // Create a new constraint system
    let cs = ConstraintSystem::<Fq>::new_ref();
    let mut rng = ark_std::test_rng();

    // Generate random points
    let g1_point = G1Projective::rand(&mut rng);
    let g2_point = G2Projective::rand(&mut rng);

    // Convert points to variables
    let g1_var = G1Var::new_witness(ark_relations::ns!(cs, "g1"), || Ok(g1_point)).unwrap();
    let g2_var = G2Var::new_witness(ark_relations::ns!(cs, "g2"), || Ok(g2_point)).unwrap();

    // Prepare points for pairing
    let g1_prepared = constraints::PairingVar::prepare_g1(&g1_var).unwrap();
    let g2_prepared = constraints::PairingVar::prepare_g2(&g2_var).unwrap();

    // Compute pairing
    let _ = constraints::PairingVar::pairing(g1_prepared, g2_prepared).unwrap();

    // Print constraint statistics
    println!("Number of constraints: {}", cs.num_constraints());
    println!("Number of instance variables: {}", cs.num_instance_variables());
    println!("Number of witness variables: {}", cs.num_witness_variables());
}