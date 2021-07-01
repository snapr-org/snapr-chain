#![cfg(feature = "runtime-benchmarks")]

use crate::{*};
use frame_benchmarking::{benchmarks, account, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use primitives::{currency::SNAPR, time::DAYS};

benchmarks! {
	where_clause { where BalanceOf<T>: From<u128> }

	on_initialize_empty {
	}: {
		// trigger regular block without era change
		Pallet::<T>::on_initialize((1 as u32).into());
	}

	on_initialize_era {
		// benchmark election worst-case (everyone votes for everyone else who is a valid candidate)
		// TODO: commitments are weakly capped - 1k is a conservative ceiling. re-run if surpassed
		// TODO: if MaxMembers changes (number of winners), this benchmark should re-run
		let c in 0..1_000;
		for i in 0..1_000 {
			let voter: T::AccountId = account("voter", i, 0);
			let candidate: T::AccountId = account("candidate", i, 0);
			T::Currency::deposit_creating(&voter, BalanceOf::<T>::from(100_001 * SNAPR));
			T::Currency::deposit_creating(&candidate, BalanceOf::<T>::from(1_000_001 * SNAPR));

			let _ = Pallet::<T>::start_candidacy(
				RawOrigin::Signed(candidate.clone()).into()
			);

			let amount: BalanceOf<T> = BalanceOf::<T>::from(100_000 * SNAPR);
			let _ = Pallet::<T>::commit(
				RawOrigin::Signed(voter.clone()).into(),
				amount,
				LockDuration::OneYear,
				candidate
			);
		}

	}: {
		// trigger the era change block
		Pallet::<T>::on_initialize((7 * DAYS).into());
	}

	start_candidacy {
		let trillian: T::AccountId = account("trillian", 0, 0);

		// trillian needs funds
		let deposit: BalanceOf<T> = BalanceOf::<T>::from(1_000_001 * SNAPR);
		T::Currency::deposit_creating(&trillian, deposit);

	}: _(RawOrigin::Signed(trillian))

	stop_candidacy {
		let trillian: T::AccountId = account("trillian", 0, 0);

		// trillian needs funds
		let deposit: BalanceOf<T> = BalanceOf::<T>::from(1_000_001 * SNAPR);
		T::Currency::deposit_creating(&trillian, deposit);

		let _ = Pallet::<T>::start_candidacy(
			RawOrigin::Signed(trillian.clone()).into(),
		);

	}: _(RawOrigin::Signed(trillian))

	commit {
		let trillian: T::AccountId = account("trillian", 0, 0);
		let ford: T::AccountId = account("ford", 0, 0);

		// trillian needs funds
		let deposit: BalanceOf<T> = BalanceOf::<T>::from(100_001 * SNAPR);
		T::Currency::deposit_creating(&trillian, deposit);

		let amount: BalanceOf<T> = BalanceOf::<T>::from(100_000 * SNAPR);
	}: _(RawOrigin::Signed(trillian), amount, LockDuration::OneYear, ford)


	add_funds {
		let trillian: T::AccountId = account("trillian", 0, 0);
		let ford: T::AccountId = account("ford", 0, 0);

		// trillian needs funds
		let deposit: BalanceOf<T> = BalanceOf::<T>::from(200_001 * SNAPR);
		T::Currency::deposit_creating(&trillian, deposit);

		let amount: BalanceOf<T> = BalanceOf::<T>::from(100_000 * SNAPR);

		// she makes initial commitment
		let _ = Pallet::<T>::commit(
			RawOrigin::Signed(trillian.clone()).into(),
			amount,
			LockDuration::OneYear,
			ford
		);

	}: _(RawOrigin::Signed(trillian), amount)

	unbond {
		let trillian: T::AccountId = account("trillian", 0, 0);
		let ford: T::AccountId = account("ford", 0, 0);

		// trillian needs funds
		let deposit: BalanceOf<T> = BalanceOf::<T>::from(200_001 * SNAPR);
		T::Currency::deposit_creating(&trillian, deposit);

		let amount: BalanceOf<T> = BalanceOf::<T>::from(100_000 * SNAPR);

		// she makes initial commitment
		let _ = Pallet::<T>::commit(
			RawOrigin::Signed(trillian.clone()).into(),
			amount,
			LockDuration::OneYear,
			ford
		);

	}: _(RawOrigin::Signed(trillian))

	withdraw {
		let trillian: T::AccountId = account("trillian", 0, 0);
		let ford: T::AccountId = account("ford", 0, 0);

		// trillian needs funds
		let deposit: BalanceOf<T> = BalanceOf::<T>::from(200_001 * SNAPR);
		T::Currency::deposit_creating(&trillian, deposit);

		let amount: BalanceOf<T> = BalanceOf::<T>::from(100_000 * SNAPR);

		// she makes initial commitment
		let _ = Pallet::<T>::commit(
			RawOrigin::Signed(trillian.clone()).into(),
			amount,
			LockDuration::OneMonth,
			ford
		);

		// she unbonds
		let _ = Pallet::<T>::unbond(
			RawOrigin::Signed(trillian.clone()).into(),
		);

		// skip 1 month
		frame_system::Module::<T>::set_block_number((31 * DAYS).into());

	}: _(RawOrigin::Signed(trillian))

	vote_candidate {
		let trillian: T::AccountId = account("trillian", 0, 0);
		let ford: T::AccountId = account("ford", 0, 0);
		let charlie: T::AccountId = account("charlie", 0, 0);

		// trillian needs funds
		let deposit: BalanceOf<T> = BalanceOf::<T>::from(200_001 * SNAPR);
		T::Currency::deposit_creating(&trillian, deposit);

		let amount: BalanceOf<T> = BalanceOf::<T>::from(100_000 * SNAPR);

		// she makes initial commitment
		let _ = Pallet::<T>::commit(
			RawOrigin::Signed(trillian.clone()).into(),
			amount,
			LockDuration::OneYear,
			ford
		);

	}: _(RawOrigin::Signed(trillian), charlie)

}

// auto-generate benchmark tests
impl_benchmark_test_suite!(Pallet, mock::new_test_ext(), mock::Runtime);

