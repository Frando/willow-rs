#![no_main]

use libfuzzer_sys::fuzz_target;
use ufotofu::local_nb::consumer::TestConsumer;
use willow_data_model::encoding::unsigned_int::U64BE;
use willow_data_model_fuzz::encoding_roundtrip;

fuzz_target!(|data: (u64, TestConsumer<u8, u16, ()>)| {
    let (n, mut consumer) = data;

    smol::block_on(async {
        encoding_roundtrip::<_, TestConsumer<u8, u16, ()>>(U64BE::from(n), &mut consumer).await;
    });
});
