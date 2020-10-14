use std::fs::File;
use std::io::BufWriter;
use std::process;
use std::f64::consts::{PI};
use std::process::Command;
use std::fs;



fn main() {
    fs::create_dir_all("tmp").unwrap();
        
    let width: f64 = 720.0;
    let height: f64 = 480.0;
    let fps: f64 = 30.0;
    let period: f64 = 1.0/fps;
    let half_width = width/2.0;
    let half_height = height/2.0;

    
    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, width as i32, height as i32).unwrap();
    let cr = cairo::Context::new(&surface);
    
    let grad = &cairo::RadialGradient::new(
      half_width,
      half_height,
      1.0,
      half_width,
      half_height,
      half_height
    );
    
    grad.add_color_stop_rgba(0.0, 1.0, 1.0, 1.0, 1.0);
    grad.add_color_stop_rgba(1.0, 0.9, 0.9, 0.9, 1.0);
    

    

    
    let handle = match librsvg::Loader::new().read_path("res/example.svg") { 
      Ok(handle) => handle,

      Err(e) => {
          eprintln!("loading error: {}", e);
          process::exit(1);
      }
    };
    
    let svg_renderer = librsvg::CairoRenderer::new(&handle);
    

    


    
    for i in 0..90 {
      cr.rectangle(0.0, 0.0, width, height);
      cr.set_source(grad);
      cr.fill();
      
      let frame = i as f64;
      
      let t = period*frame;
      let a = width/8.0;
      let f = 0.5;
      
      let res = svg_renderer.render_document(&cr, &cairo::Rectangle {
        x: half_width + a*f64::sin(2.0*PI*f*t),
        y: 50.0,
        width: 200.0,
        height: 200.0,
      });
      

      match res {
          Ok(()) => {
              let s = format!("tmp/{:04}.png", i);
              let mut file = BufWriter::new(File::create(s).unwrap());

              surface.write_to_png(&mut file).unwrap();
              
              // let data = surface.get_data().unwrap();
              
          }

          Err(e) => {
              eprintln!("rendering error: {}", e);
              process::exit(1);
          }
      }
    }
    
    let ffmpeg_args = [
      // "-f", "image2pipe", 
      "-s", "720x480", 
      "-r", "30", 
      // "-i", "-", 
      "-i", "tmp/%04d.png", 
      "-vf", "format=yuv420p", 
      "-vcodec", "libx264", 
      "-profile:v", "high", 
      "-preset:v", "medium", 
      "-crf", "18", 
      "-movflags", "faststart", 
      "-y", "out.mp4"
    ];
    
    let mut child = Command::new("ffmpeg")
      .args(&ffmpeg_args)
      .spawn()
      .expect("failed to execute child");
      
    let ecode = child.wait()
                 .expect("failed to wait on child");

    assert!(ecode.success());
    
}
