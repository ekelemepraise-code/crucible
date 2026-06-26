//! Simulated transaction dry-runs and fee estimation.
//!
//! **Host-only:** [`SimulatedTx`] is constructed by [`MockEnv::simulate`] and
//! depends on the Soroban host test utilities. It is intended exclusively for
//! use in `#[cfg(test)]` contexts on the host.
//!
//! [`MockEnv::simulate`]: crate::env::MockEnv::simulate
/// without committing the state changes.
use soroban_sdk::Address;


pub struct SimulatedTx<T> {
    fee: i64,
    instructions: u64,
    required_auths: Vec<Address>,
    success: bool,
    result: Option<T>,
    re_run: Option<Box<dyn FnOnce() -> T>>,
}

impl<T> SimulatedTx<T> {
    /// Internal constructor for `MockEnv`.
    pub(crate) fn new(
        fee: i64,
        instructions: u64,
        required_auths: Vec<Address>,
        success: bool,
        result: Option<T>,
        re_run: Option<Box<dyn FnOnce() -> T>>,
    ) -> Self {
        Self {
            fee,
            instructions,
            required_auths,
            success,
            result,
            re_run,
        }
    }

    /// Returns the estimated network fee in stroops.
    pub fn fee(&self) -> i64 {
        self.fee
    }

    /// Returns the total instruction count consumed by the call.
    pub fn instructions(&self) -> u64 {
        self.instructions
    }

    /// Returns the list of addresses that required authorization during the call.
    pub fn required_auths(&self) -> Vec<Address> {
        self.required_auths.clone()
    }

    /// Returns whether the transaction would succeed if committed.
    pub fn would_succeed(&self) -> bool {
        self.success
    }

    /// Returns the result of the call if it succeeded, or `None` if it failed.
    pub fn result(&self) -> Option<&T> {
        self.result.as_ref()
    }

    /// Re-runs the call and commits the state changes.
    ///
    /// # Panics
    ///
    /// Panics if the transaction would not succeed or if `commit()` has already been called.
    pub fn commit(mut self) -> T {
        if !self.would_succeed() {
            panic!("Cannot commit a failed transaction simulation.");
        }

        let re_run = self
            .re_run
            .take()
            .expect("Transaction already committed or closure was consumed.");
        re_run()
    }
}
