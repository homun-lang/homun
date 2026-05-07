// Bin entry: delegates to main_hom::main(), compiled from src/main.hom.
// Cargo's [[bin]] needs a stable .rs path; main.hom compiles to OUT_DIR which
// Cargo can't reference, so this 4-line shim sits in src/ as the bin target.
fn main() {
    homunc::main_hom::main();
}
