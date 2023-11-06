use std::env;
use wasi_nn;
use serde_json::json;

fn main() {
    let args: Vec<String> = env::args().collect();
    let model_name: &str = &args[1];
    let prompt: &str = &args[2];

    let graph =
        wasi_nn::GraphBuilder::new(wasi_nn::GraphEncoding::Ggml, wasi_nn::ExecutionTarget::AUTO)
            .build_from_cache(model_name)
            .expect("Failed to load the model");

    let mut context = graph.init_execution_context().expect("Failed to init context");
    let tensor_data = prompt.as_bytes().to_vec();
    let options = json!({
        "stream-stdout": true,
        "enable-log": true,
    });
    context
        .set_input(
            1,
            wasi_nn::TensorType::U8,
            &[1],
            &options.to_string().as_bytes().to_vec(),
        )
        .unwrap();
    context
        .set_input(0, wasi_nn::TensorType::U8, &[1], &tensor_data)
        .expect("Failed to set prompt as the input tensor");

    println!("{}", prompt);
    context.compute().expect("Failed to complete inference");
}
