use ark_ec::pairing::Pairing;
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSystem, SynthesisError};
use ark_std::UniformRand;

#[cfg(test)]
mod tests {
    use super::*;
    use ark_mnt4_298::{constraints as mnt4_constraints, Fq as Mnt4Fq, G1Projective as Mnt4G1, G2Projective as Mnt4G2, MNT4_298};
    use ark_mnt6_298::{constraints as mnt6_constraints, Fq as Mnt6Fq, G1Projective as Mnt6G1, G2Projective as Mnt6G2, MNT6_298};

    #[test]
    fn test_mnt4_298_constraints() -> Result<(), SynthesisError> {
        let cs = ConstraintSystem::<Mnt4Fq>::new_ref();
        let mut rng = ark_std::test_rng();

        println!("\nMNT4-298 Initial constraints: {}", cs.num_constraints());

        // Generate random points
        let g1_native = Mnt4G1::rand(&mut rng);
        let g2_native = Mnt4G2::rand(&mut rng);

        // Create witness variables
        let g1_var = mnt4_constraints::G1Var::new_witness(ark_relations::ns!(cs, "g1"), || Ok(g1_native))?;
        let g2_var = mnt4_constraints::G2Var::new_witness(ark_relations::ns!(cs, "g2"), || Ok(g2_native))?;

        // Create constant variables
        let g1_const = mnt4_constraints::G1Var::new_constant(ark_relations::ns!(cs, "g1_const"), g1_native)?;
        let g2_const = mnt4_constraints::G2Var::new_constant(ark_relations::ns!(cs, "g2_const"), g2_native)?;

        println!("After variable allocation: {}", cs.num_constraints());

        // Compute native pairing for verification
        let pairing_result_native = MNT4_298::pairing(g1_native, g2_native);

        // Prepare witness points and measure constraints
        let prep_constraints_before = cs.num_constraints();
        let g1_prep = mnt4_constraints::PairingVar::prepare_g1(&g1_var)?;
        println!("G1 preparation constraints: {}", cs.num_constraints() - prep_constraints_before);

        let g2_prep = mnt4_constraints::PairingVar::prepare_g2(&g2_var)?;
        println!("G2 preparation constraints: {}", cs.num_constraints() - cs.num_constraints());

        // Compute pairing
        let pairing_start = cs.num_constraints();
        let pairing_result = mnt4_constraints::PairingVar::pairing(g1_prep, g2_prep)?;
        println!("Pairing computation constraints: {}", cs.num_constraints() - pairing_start);

        // Verify correctness
        assert_eq!(pairing_result.value()?, pairing_result_native.0);
        assert!(cs.is_satisfied()?);

        // Compare with constant pairing
        println!("\nComputing constant pairing for comparison...");
        let cs_before_const = cs.num_constraints();
        
        let g1_prep_const = mnt4_constraints::PairingVar::prepare_g1(&g1_const)?;
        let g2_prep_const = mnt4_constraints::PairingVar::prepare_g2(&g2_const)?;
        let pairing_result_const = mnt4_constraints::PairingVar::pairing(g1_prep_const, g2_prep_const)?;
        
        println!("Constant pairing constraints: {}", cs.num_constraints() - cs_before_const);

        // Final statistics
        println!("\nFinal Statistics:");
        println!("Total constraints: {}", cs.num_constraints());
        println!("Witness variables: {}", cs.num_witness_variables());
        println!("Instance variables: {}", cs.num_instance_variables());

        Ok(())
    }

    #[test]
    fn test_mnt6_298_constraints() -> Result<(), SynthesisError> {
        let cs = ConstraintSystem::<Mnt6Fq>::new_ref();
        let mut rng = ark_std::test_rng();

        println!("\nMNT6-298 Initial constraints: {}", cs.num_constraints());

        // Generate random points
        let g1_native = Mnt6G1::rand(&mut rng);
        let g2_native = Mnt6G2::rand(&mut rng);

        // Create witness variables
        let g1_var = mnt6_constraints::G1Var::new_witness(ark_relations::ns!(cs, "g1"), || Ok(g1_native))?;
        let g2_var = mnt6_constraints::G2Var::new_witness(ark_relations::ns!(cs, "g2"), || Ok(g2_native))?;

        // Create constant variables
        let g1_const = mnt6_constraints::G1Var::new_constant(ark_relations::ns!(cs, "g1_const"), g1_native)?;
        let g2_const = mnt6_constraints::G2Var::new_constant(ark_relations::ns!(cs, "g2_const"), g2_native)?;

        println!("After variable allocation: {}", cs.num_constraints());

        // Compute native pairing for verification
        let pairing_result_native = MNT6_298::pairing(g1_native, g2_native);

        // Prepare witness points and measure constraints
        let prep_constraints_before = cs.num_constraints();
        let g1_prep = mnt6_constraints::PairingVar::prepare_g1(&g1_var)?;
        println!("G1 preparation constraints: {}", cs.num_constraints() - prep_constraints_before);

        let g2_prep = mnt6_constraints::PairingVar::prepare_g2(&g2_var)?;
        println!("G2 preparation constraints: {}", cs.num_constraints() - cs.num_constraints());

        // Compute pairing
        let pairing_start = cs.num_constraints();
        let pairing_result = mnt6_constraints::PairingVar::pairing(g1_prep, g2_prep)?;
        println!("Pairing computation constraints: {}", cs.num_constraints() - pairing_start);

        // Verify correctness
        assert_eq!(pairing_result.value()?, pairing_result_native.0);
        assert!(cs.is_satisfied()?);

        // Compare with constant pairing
        println!("\nComputing constant pairing for comparison...");
        let cs_before_const = cs.num_constraints();
        
        let g1_prep_const = mnt6_constraints::PairingVar::prepare_g1(&g1_const)?;
        let g2_prep_const = mnt6_constraints::PairingVar::prepare_g2(&g2_const)?;
        let pairing_result_const = mnt6_constraints::PairingVar::pairing(g1_prep_const, g2_prep_const)?;
        
        println!("Constant pairing constraints: {}", cs.num_constraints() - cs_before_const);

        // Final statistics
        println!("\nFinal Statistics:");
        println!("Total constraints: {}", cs.num_constraints());
        println!("Witness variables: {}", cs.num_witness_variables());
        println!("Instance variables: {}", cs.num_instance_variables());

        Ok(())
    }
}