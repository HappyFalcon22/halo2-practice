use eth_types::Error;
use ff::Field;
use halo2_proofs::circuit::{AssignedCell, Layouter, Value};
use halo2_proofs::plonk::{Advice, Column, ConstraintSystem, Instance};
extern crate std;
use std::io::Result;
use std::marker::PhantomData;
// Problem specification
// Suppose we have a list of usernames and balances (need to keep private).
// We receive a tuple (username, balance) from a verifier
// and we prove that this user exists in the list and his balance is corrrect.

// Inclusion check circuit
#[derive(Debug, Clone)]
pub struct InclusionCheckConfig {
    // 2 advice columns for username and balance
    pub advice: [Column<Advice>; 2],
    // 1 column of instance
    pub instance: Column<Instance>,
}
#[derive(Debug, Clone)]
pub struct InclusionCheckChip<F: Field> {
    config: InclusionCheckConfig,
    // Since the config does not use the generic trait F
    //, we must have another "do nothing" member that uses F.
    // For this problem, we have the PhantomData type.
    _marker: PhantomData<F>,
}

// Implement the inclusion check chip, including:
// + Construct.
// + Configure.
// + Assign cells.
// + Expose public.
impl<F: Field> InclusionCheckChip<F> {
    // Construct: use the config to construct the chip.
    pub fn construct(config: InclusionCheckConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    // Configure: use meta: ConstraintSystem to build the configuration with equality constraints and gate constraints
    // Input: Configurations components, a meta variable
    // Output: Config struct.
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; 2],
        instance: Column<Instance>,
    ) -> InclusionCheckConfig {
        // Get username and balance columns from advice
        let usernames = advice[0];
        let balances = advice[1];

        // Enable permutation check for a column: enable_equality
        meta.enable_equality(usernames);
        meta.enable_equality(balances);
        // Need permutation check on instance column as well
        meta.enable_equality(instance);

        // Done!
        InclusionCheckConfig {
            advice: [usernames, balances],
            instance,
        }
    }

    // Assign username and balance columns to the region
    pub fn assign_generic_rows(
        &self,
        mut layouter: impl Layouter<F>,
        username: Value<F>,
        balance: Value<F>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "generic_row",
            |mut region| {
                region.assign_advice(|| "username", self.config.advice[0], 0, username)?;
                region.assign_advice(|| "balance", self.config.advice[1], 0, balance)?;
                Ok(())
            },
        )
    }

    //
    pub fn assign_inclusion_check_row(
        mut layouter: impl Layouter<F>,
        username: Value<F>,
        balance: Value<F>,
    ) -> Result<(AssignedCell<F, F>, AssignedCell<F, F>), Error> {
        layouter.assign_region(
            || "inclusion_check_row",
            |mut region| {
                let username_cell =
                    region.assign_advice(|| "username", self.config.advice[0], 0, username)?;
                let balance_cell =
                    region.assign_advice(|| "balance", self.config.advice[1], 0, balance)?;
                region.assign_instance(|| "instance", self.config.instance)?;
                Ok((username_cell, balance_cell))
            },
        )
    }

    // Expose public
    pub fn expose_public(
        &self,
        mut layouter: impl Layouter<F>,
        pub_username: &AssignedCell<F, F>,
        pub_balance: &AssignedCell<F, F>,
    ) -> Result<(), Error> {
        // enforce equality between public_username_cell and instance column at row 0
        layouter.constrain_instance(pub_username.cell(), self.config.instance, 0)?;
        // enforce equality between balance_username_cell and instance column at row 1
        layouter.constrain_instance(pub_balance.cell(), self.config.instance, 1)?;
        Ok(())
    }
}
