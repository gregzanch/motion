use std::fs::File;
use std::io::BufWriter;
use std::process;


fn main() {
    
    let _parsed = json::parse(r#"

    {
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    }
    
    "#).unwrap();
    
    let width: i32 = 720;
    let height: i32 = 480;
    let half_width = f64::from(width)/2.0;
    let half_height = f64::from(height)/2.0;

    

    
    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, width, height).unwrap();
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
    
    cr.rectangle(0.0, 0.0, f64::from(width), f64::from(height));
    cr.set_source(grad);
    cr.fill();
    
    
    
    let handle = match librsvg::Loader::new().read_path("res/example.svg") {
      Ok(handle) => handle,

      Err(e) => {
          eprintln!("loading error: {}", e);
          process::exit(1);
      }
  };
  

    
    let svg_renderer = librsvg::CairoRenderer::new(&handle);
    
    let res = svg_renderer.render_document(&cr, &cairo::Rectangle {
      x: 50.0,
      y: 50.0,
      width: 200.0,
      height: 200.0,
    });
    
    

    
    match res {
        Ok(()) => {
            let mut file = BufWriter::new(File::create("out.png").unwrap());
            
            surface.write_to_png(&mut file).unwrap();
        }

        Err(e) => {
            eprintln!("rendering error: {}", e);
            process::exit(1);
        }
    }
}
