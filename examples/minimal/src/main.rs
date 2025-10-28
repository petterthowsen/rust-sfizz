use sfizz::Synth;

fn main() {
    let synth = match Synth::new() {
        Ok(synth) => synth,
        Err(err) => {
            eprintln!("Failed to create sfizz synth: {err}");
            return;
        }
    };

    println!("Synth allocated at {:?}", synth.as_raw());
}
