use crate::evm_impl::CallResult;
use crate::evm_impl::PreparedCall;
use crate::handler::Handler;
use crate::interpreter::{
    analysis::to_analysed, gas, return_ok, CallContext, CallInputs, CallScheme, Contract,
    CreateInputs, Gas, Host, InstructionResult, Interpreter, SelfDestructResult, Transfer,
};
use crate::journaled_state::{is_precompile, JournalCheckpoint, JournaledState};
use crate::primitives::{
    keccak256, Address, AnalysisKind, Bytecode, Bytes, EVMError, EVMResult, Env,
    InvalidTransaction, Log, Output, Spec, SpecId::*, TransactTo, B256, U256,
};
use crate::{db::Database, precompile, Inspector};
use crate::{inspector_instruction, EVMData};
use alloc::boxed::Box;

/// Execute EVM call.
pub fn call<DB: Database>(data: &mut EVMData<'_, DB>, gas: &Gas) -> CallResult {
    CallResult::default()
}
