use alloy_primitives::Address;
use alloy_sol_types::{SolCall, SolInterface};
use revm::InMemoryDB;
use revm::primitives::Env;

use defi_abi::uniswap3::IUniswapV3Pool;
use loom_utils::evm::evm_call;

pub struct UniswapV3StateReader {}

impl UniswapV3StateReader {
    pub fn factory(db: &InMemoryDB, env: Env, pool: Address) -> eyre::Result<Address> {
        let call_data_result = evm_call(db, env, pool, IUniswapV3Pool::IUniswapV3PoolCalls::factory(IUniswapV3Pool::factoryCall {}).abi_encode())?.0;
        let call_return = IUniswapV3Pool::factoryCall::abi_decode_returns(&call_data_result, false)?;
        Ok(call_return._0)
    }

    pub fn token0(db: &InMemoryDB, env: Env, pool: Address) -> eyre::Result<Address> {
        let call_data_result = evm_call(db, env, pool, IUniswapV3Pool::IUniswapV3PoolCalls::token0(IUniswapV3Pool::token0Call {}).abi_encode())?.0;
        let call_return = IUniswapV3Pool::token0Call::abi_decode_returns(&call_data_result, false)?;
        Ok(call_return._0)
    }

    pub fn token1(db: &InMemoryDB, env: Env, pool: Address) -> eyre::Result<Address> {
        let call_data_result = evm_call(db, env, pool, IUniswapV3Pool::IUniswapV3PoolCalls::token1(IUniswapV3Pool::token1Call {}).abi_encode())?.0;
        let call_return = IUniswapV3Pool::token1Call::abi_decode_returns(&call_data_result, false)?;
        Ok(call_return._0)
    }

    pub fn fee(db: &InMemoryDB, env: Env, pool: Address) -> eyre::Result<u32> {
        let call_data_result = evm_call(db, env, pool, IUniswapV3Pool::IUniswapV3PoolCalls::fee(IUniswapV3Pool::feeCall {}).abi_encode())?.0;
        let call_return = IUniswapV3Pool::feeCall::abi_decode_returns(&call_data_result, false)?;
        Ok(call_return._0)
    }

    pub fn tick_spacing(db: &InMemoryDB, env: Env, pool: Address) -> eyre::Result<u32> {
        let call_data_result = evm_call(db, env, pool, IUniswapV3Pool::IUniswapV3PoolCalls::tickSpacing(IUniswapV3Pool::tickSpacingCall {}).abi_encode())?.0;
        let call_return = IUniswapV3Pool::tickSpacingCall::abi_decode_returns(&call_data_result, false)?;
        Ok(call_return._0 as u32)
    }


    /*


     */
}
