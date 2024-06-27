fn main() {
    match std::env::var("PROFILE") {
        Ok(v) if v == "release" => static_vcruntime::metabuild(),
        _ => {}
    }
}