use super::*;

#[allow(unused)]
use crate::Pallet as UseBenchmarkingDemo;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	set_student_info {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}:{
		let _ = UseBenchmarkingDemo::<T>::set_student_info(RawOrigin::Signed(caller).into(), s.into(), Default::default());
	}
	verify {
		assert_eq!(<StudentsInfo<T>>::get::<<T as pallet::Config>::StudentNumberType>(s.into()), Default::default());
	}

	do_some_work {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}:{
		let _ = UseBenchmarkingDemo::<T>::do_some_work(s.into());
	}

	impl_benchmark_test_suite!(UseBenchmarkingDemo, crate::mock::new_test_ext(), crate::mock::Test);
}
