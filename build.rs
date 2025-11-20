fn main() {
    protobuf_codegen::CodeGen::new()
        .inputs(["record.proto", "session.proto", "user.proto"])
        .include("src/protos")
        .dependency(protobuf_well_known_types::get_dependency(
            "protobuf_well_known_types",
        ))
        .generate_and_compile()
        .unwrap();
}
