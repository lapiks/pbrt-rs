use pbrt_rs::{
    cpu::render::render_cpu, options::PBRTOptions, parser::parse_files, pbrt::{clean_up_pbrt, init_pbrt}, scene::{BasicScene, BasicSceneBuilder}, wavefront::render_wavefront
};

fn main() {
    // Convert command-line arguments to vector of strings
    // Declare variables for parsed command line
    let options = PBRTOptions { use_gpu: todo!(), wavefront: todo!() };
    // Process command-line arguments
    // Initialize pbrt
    init_pbrt(&options);
    // Parse provided scene description files
    let scene = BasicScene {};
    let builder = BasicSceneBuilder {  };
    parse_files(&builder, &scene);
    // Render the scene
    if options.use_gpu || options.wavefront {
        render_wavefront(&scene);        
    } else {
        render_cpu(&scene);
    }
    // Clean up after rendering the scene
    clean_up_pbrt();
}