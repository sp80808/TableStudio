fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    // Default to a higher period size to prevent CPAL panics on macOS (where CoreAudio 
    // might deliver awkward block sizes like 558 instead of the requested 512).
    if !args.iter().any(|arg| arg == "--period-size" || arg == "-p") {
        args.push("--period-size".to_string());
        args.push("1024".to_string());
    }
    
    nih_plug::wrapper::standalone::nih_export_standalone_with_args::<wavetable_designer::WavetableDesigner, _>(args);
}
