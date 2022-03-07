#![feature(once_cell)]

use tract_onnx::prelude::*;

use std::lazy::SyncLazy;

pub static MODEL: SyncLazy<RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>> = SyncLazy::new(|| {
    let model = tract_onnx::onnx()
        .model_for_path("check.onnx").unwrap()
        .with_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 11, 1))).unwrap()
        .into_optimized().unwrap()
        .into_runnable().unwrap();
    model
});

pub static PARAMETER_STANDARD_DEVIATIONS: SyncLazy<tract_ndarray::Array1<f32>> = SyncLazy::new(|| {
    tract_ndarray::arr1(&[
        0.28133126679885656, 0.28100480365686287, 0.28140136435474244, 0.907001394792043, 1.811683338833852, 0.2815981892528909, 0.281641754864262, 0.28109705707606697, 0.9062620846468298, 1.8139690831565327, 2.886950440590801
    ])
});

pub static PARAMETER_MEANS: SyncLazy<tract_ndarray::Array1<f32>> = SyncLazy::new(|| {
    tract_ndarray::arr1(&[
        -0.0008009571736463096, -0.0008946310379428422, -2.274708783534052e-05, 1.5716876559520705, 3.1388159291733086, -0.001410436081400537, -0.0001470613574040905, -3.793528434430451e-05, 1.5723036365564083, 3.1463088925150258, 5.509554132916939
    ])
});

pub static PHASE_AMPLITUDE_MEAN: f32 = 34025.080543335825;
pub static PHASE_AMPLITUDE_STANDARD_DEVIATION: f32 = 47698.66676993027;
