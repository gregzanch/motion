use std::{f64::consts::PI, process::{self, Stdio}};
use std::io::Write;
use std::process::Command;
use cairo::{ImageSurface, Format, Context};

mod motion_renderer;
use motion_renderer::MotionRenderer;

mod x264_presets;


fn clear_canvas(cr: &Context){
    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.paint();
}


fn draw(surface: &ImageSurface, frame: i32, motion_renderer: &MotionRenderer, svg_renderers: &Vec<&librsvg::CairoRenderer>) {

    let t = motion_renderer.period * f64::from(frame);

    let cr = Context::new(surface);
    
    let width = motion_renderer.width;
    let height =  motion_renderer.height;

    // cr.set_source_rgb(1.0, 1.0, 1.0);
    // cr.paint();
    clear_canvas(&cr);
    // cr.set_source(grad);
    // cr.rectangle(0.0, 0.0, width, height);
    // cr.fill();
    // cr.paint();

    let cx =  width / 2.0;
    let cy = height / 2.0;
    let r = cy;
    let cstart = -0.5 * PI;
    let cend = cstart + 2.0 * PI * ((frame + 1) as f64) / 300.0;
    cr.move_to(cx, cy);
    cr.line_to(cx, 0.0);
    cr.arc(cx, cy, r, cstart, cend);
    cr.line_to(cx, cy);
    cr.set_source_rgba(0.0, 0.5, 0.0, 0.2);
    cr.fill();

    cr.select_font_face(
        "sans-serif",
        cairo::FontSlant::Normal,
        cairo::FontWeight::Normal,
    );
    cr.set_font_size(70.0);
    cr.move_to(600.0 - f64::from(frame), 100.0);
    cr.set_source_rgb(0.0, 0.0, 1.0);
    let s = format!("{}", frame);
    cr.show_text(&s);
    cr.fill();




    
    let a = motion_renderer.width / 8.0;
    let f = 0.5;
    let mut i = 0;
    for r in svg_renderers {
        i=i+1;
        let res = r.render_document(
            &cr,
            &cairo::Rectangle {
                x: width/2.0 + a * f64::sin(2.0 * PI * f * t),
                y: 50.0 * f64::from(i),
                width: 200.0,
                height: 200.0,
            },
        );


        match res {
            Ok(()) => {}

            Err(e) => {
                eprintln!("rendering error: {}", e);
                process::exit(1);
            }
        }

    }

}




struct MovieArgs<'a> {
    width: i32,
    height: i32,
    framerate: f64,
    crf: i32,
    filename: &'a str,
    duration: f64,
    preset: &'a str,
}

fn make_ffmpeg_command(args: MovieArgs) -> String {
    format!(
        "ffmpeg -f rawvideo -pix_fmt bgra -s {width}x{height} -i - -pix_fmt yuv420p -crf {crf} -preset:v {preset} -r {framerate} -y {filename}",
        width = args.width,
        height = args.height,
        crf = args.crf,
        preset = args.preset,
        framerate = args.framerate,
        filename = args.filename,
    )
}



fn make_movie(args: MovieArgs) {
    let frames = (args.framerate * args.duration) as i32;

    let motion_renderer = MotionRenderer::new(
        f64::from(args.width),
        f64::from(args.height),
        args.framerate
    );

    let mut surface = ImageSurface::create(
        Format::ARgb32, 
        args.width, 
        args.height
    ).expect("Couldn't create surface");

    // let mut img_surface = ImageSurface::create_from_png(&mut BufReader::new(File::open("res/ex.png").unwrap())).expect("Couldn't create surface");

    // let d = img_surface.get_data().expect("Failed to getdata");
    
    // let cr = cairo::Context::new(&img_surface);



    let handle = match librsvg::Loader::new().read_path("res/ex.svg") {
        Ok(handle) => handle,

        Err(e) => {
            eprintln!("loading error: {}", e);
            std::process::exit(1);
        }
    };

    let r1 = &librsvg::CairoRenderer::new(&handle);

    let svg_renderers = vec![r1];

    let mut child = Command::new("/bin/sh")
        .args(&["-c", &make_ffmpeg_command(args)])
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    {
        // limited borrow of stdin
        let child_stdin = child.stdin.as_mut().expect("failed to get stdin");
        
        (0..frames).for_each(|frame| {
            draw(&surface, frame, &motion_renderer, &svg_renderers);


            let d = surface.get_data().expect("Failed to get_data");
            child_stdin.write_all(&d).expect("Failed to write to file");
        });
    }
    child.wait().expect("child process wasn't running");
}


fn main() {
    make_movie(MovieArgs {
        filename: "out/out.mp4", 
        width: 1280, 
        height: 720, 
        framerate: 30.0, 
        duration: 3.0,
        crf: 23,
        preset: x264_presets::ULTRAFAST
    });
    // make_movie("out/out.webm", 960, 480, 30, 90);
    // fs::create_dir_all("tmp").unwrap();


    // let surface =
    //     cairo::ImageSurface::create(cairo::Format::ARgb32, width as i32, height as i32).unwrap();
    // let cr = cairo::Context::new(&surface);

    // let grad = &cairo::RadialGradient::new(
    //     half_width,
    //     half_height,
    //     1.0,
    //     half_width,
    //     half_height,
    //     half_height,
    // );

    // grad.add_color_stop_rgba(0.0, 1.0, 1.0, 1.0, 1.0);
    // grad.add_color_stop_rgba(1.0, 0.9, 0.9, 0.9, 1.0);

   

    // // cr.rectangle(0.0, 0.0, width, height);
    // cr.set_source(grad);
    // cr.rectangle(0.0, 0.0, width, height);
    // cr.fill();
    // cr.paint();
    
    // for i in 0..90 {

        
        

    // }



    // let ffmpeg_args = [
    //     // "-f", "image2pipe",
    //     "-s", "720x480",
    //     "-r", "30",
    //     // "-i", "-",
    //     "-i", "tmp/%04d.png",
    //     "-vf", "format=yuv420p",
    //     "-vcodec", "libx264",
    //     "-profile:v", "high",
    //     "-preset:v", "medium",
    //     "-crf", "18",
    //     "-movflags", "faststart",
    //     "-y", "out.mp4",
    // ];

    // let mut child = Command::new("ffmpeg")
    //     .args(&ffmpeg_args)
    //     .spawn()
    //     .expect("failed to execute child");

    // let ecode = child.wait().expect("failed to wait on child");

    // assert!(ecode.success());

    
}
