use cairo::{Context, Format, ImageSurface, Rectangle};
use rsvg::{Loader, CairoRenderer,};
#[derive(Clone)]
pub struct LoadSvg{
    width: i32,
    height: i32,
}

impl LoadSvg{
    pub fn new(w: i32, h: i32) -> Self{
        
        LoadSvg{
            width: w,
            height: h,
        }
    }

    pub fn get_texture_for_png(&self, path: String) -> Vec<u8>{
        let surface =  self.get_render_surface(path);
        let mut buf: Vec<u8> = Vec::new();
        let _ = surface.write_to_png(&mut buf);
        //Bytes::from_owned(buf)
        buf
    }

    fn get_render_surface(&self, path: String) -> ImageSurface{

        let handle = Loader::new() .read_path(path) .expect("dont load SVG");
        let surface = ImageSurface::create(Format::ARgb32, self.width, self.height) .expect("dont create surface"); 
        let cr = Context::new(&surface).unwrap();
        CairoRenderer::new(&handle).render_document(&cr, &Rectangle::new(0.0, 0.0, self.width as f64, self.height as f64)) .expect("Error al renderizar");
        surface
    }
}
