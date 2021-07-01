//! Unit tests for the evm-bridge module.

#![cfg(test)]

use super::*;
use frame_support::{assert_err, assert_ok};
use mock::{trillian, ford, erc20_address, EvmBridgeModule, ExtBuilder, Runtime};
use primitives::evm::AddressMapping;

#[test]
fn should_read_total_supply() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(
			EvmBridgeModule::total_supply(InvokeContext {
				contract: erc20_address(),
				sender: Default::default(),
				origin: Default::default(),
			}),
			Ok(u128::max_value())
		);
	});
}

#[test]
fn should_read_balance_of() {
	ExtBuilder::default().build().execute_with(|| {
		let context = InvokeContext {
			contract: erc20_address(),
			sender: Default::default(),
			origin: Default::default(),
		};

		assert_eq!(EvmBridgeModule::balance_of(context, ford()), Ok(0));

		assert_eq!(EvmBridgeModule::balance_of(context, trillian()), Ok(u128::max_value()));

		assert_eq!(EvmBridgeModule::balance_of(context, ford()), Ok(0));
	});
}

#[test]
fn should_transfer() {
	ExtBuilder::default()
		.balances(vec![
			(
				<Runtime as module_evm::Config>::AddressMapping::get_account_id(&trillian()),
				100000,
			),
			(
				<Runtime as module_evm::Config>::AddressMapping::get_account_id(&ford()),
				100000,
			),
		])
		.build()
		.execute_with(|| {
			assert_err!(
				EvmBridgeModule::transfer(
					InvokeContext {
						contract: erc20_address(),
						sender: ford(),
						origin: ford(),
					},
					trillian(),
					10
				),
				Error::<Runtime>::ExecutionRevert
			);

			assert_ok!(EvmBridgeModule::transfer(
				InvokeContext {
					contract: erc20_address(),
					sender: trillian(),
					origin: trillian(),
				},
				ford(),
				100
			));
			assert_eq!(
				EvmBridgeModule::balance_of(
					InvokeContext {
						contract: erc20_address(),
						sender: trillian(),
						origin: trillian(),
					},
					ford()
				),
				Ok(100)
			);

			assert_ok!(EvmBridgeModule::transfer(
				InvokeContext {
					contract: erc20_address(),
					sender: ford(),
					origin: ford(),
				},
				trillian(),
				10
			));

			assert_eq!(
				EvmBridgeModule::balance_of(
					InvokeContext {
						contract: erc20_address(),
						sender: trillian(),
						origin: ford(),
					},
					ford()
				),
				Ok(90)
			);

			assert_err!(
				EvmBridgeModule::transfer(
					InvokeContext {
						contract: erc20_address(),
						sender: ford(),
						origin: ford(),
					},
					trillian(),
					100
				),
				Error::<Runtime>::ExecutionRevert
			);
		});
}
