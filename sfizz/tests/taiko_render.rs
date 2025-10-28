use std::path::PathBuf;

use sfizz::Synth;

fn taiko_sfz_path() -> Option<PathBuf> {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("fixtures")
        .join("sfz")
        .join("SCC Taiko Drums")
        .join("SCC Taiko Drums.sfz");
    if base.exists() {
        Some(base)
    } else {
        None
    }
}

#[test]
fn taiko_sfizz_renders_audio() {
    let Some(sfz_path) = taiko_sfz_path() else {
        eprintln!("Skipping test: SCC Taiko Drums fixture not found");
        return;
    };

    let mut synth = Synth::new().expect("create synth");
    synth.set_sample_rate(44_100.0);
    synth
        .set_block_size(512)
        .expect("block size within C API bounds");
    synth.load_sfz(&sfz_path).expect("load SCC Taiko Drums");

    let mut left = vec![0.0f32; 512];
    let mut right = vec![0.0f32; 512];

    // Prime internal state before triggering the note.
    synth
        .render_block(&mut [&mut left[..], &mut right[..]])
        .expect("prime render");

    synth.note_on(60, 100);
    synth
        .render_block(&mut [&mut left[..], &mut right[..]])
        .expect("render after note on");

    let energy: f32 = left
        .iter()
        .chain(right.iter())
        .map(|sample| sample.abs())
        .sum();

    assert!(energy > 0.0, "rendered block should contain audio energy");

    synth.note_off(60, 0);
    synth.all_sound_off();
}
