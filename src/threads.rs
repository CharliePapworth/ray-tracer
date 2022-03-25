use crate::*;
use crate::vec::*;
use crate::traceable::*;
use crate::util::*;

use std::sync::Arc;
use std::sync::Barrier;
use std::sync::mpsc::*;
use std::thread;

#[derive (Copy, Clone)]
pub struct InputData {
    pub image_width: usize,
    pub image_height: usize,
    pub samples_per_pixel: usize,
    pub max_depth: i32,
    pub camera_settings: CameraSettings,
    pub mode: DrawMode,
    pub run: bool,
    pub done: bool
}

#[derive (Copy, Clone)]
pub enum DrawMode {
    Raytrace,
    Rasterize
}
#[derive (Clone, Default)]
pub struct ImageData{
    pub pixel_colors: Vec<Color>,
    pub image_width: usize,
    pub image_height: usize,
    pub samples: usize
}

pub fn initialise_threads<H, W>(input_data: InputData, scene_data: Arc<StaticData<H, W>>, thread_to_gui_tx: Sender<ImageData>, num_threads: i32) -> Vec<Sender<InputData>>
where H: Hit + 'static, W: Outline + 'static {
    let mut senders = vec![];
    let barrier = Arc::new(Barrier::new((num_threads) as usize));
    for _ in 0..num_threads {
        let barrier_clone = Arc::clone(&barrier);
        let thread_to_gui_tx = thread_to_gui_tx.clone();
        let (gui_to_thread_tx, gui_to_thread_rx): (Sender<InputData>, Receiver<InputData>) = channel();
        let static_data = Arc::clone(&scene_data);
        thread::spawn(move || run_thread(input_data, static_data, thread_to_gui_tx,  gui_to_thread_rx, barrier_clone));
        senders.push(gui_to_thread_tx);
    }
    senders
}

 pub fn run_thread<H, W>(mut input_data: InputData, static_data: Arc<StaticData<H, W>>, thread_to_gui_tx: Sender<ImageData>, gui_to_thread_rx: Receiver<InputData>, barrier: Arc<Barrier>)
 where H: Hit +'static, W: Outline + 'static{

    while input_data.run {
        if !input_data.done {
            match input_data.mode {
                DrawMode::Raytrace => {
                    let iteration_result = raytrace(input_data, Arc::clone(&static_data), thread_to_gui_tx.clone() , &gui_to_thread_rx);
                    if let Err(new_data) = iteration_result {
                        input_data = new_data;
                        barrier.wait();
                    }
                }

                DrawMode::Rasterize => {
                    let iteration_result = rasterize(input_data, Arc::clone(&static_data), thread_to_gui_tx.clone() , &gui_to_thread_rx);
                    if let Err(new_data) = iteration_result {
                        input_data = new_data;
                        barrier.wait();
                    }
                }
            }
           
        }
        else {
            let message = gui_to_thread_rx.recv();
            match message {
                Ok(new_data) => input_data = new_data,
                Err(_) => input_data.run = false
            }
        }
    }
 }

 pub fn rasterize<H, W>(mut input_data: InputData, static_data: Arc<StaticData<H, W>>, thread_to_gui_tx: Sender<ImageData>, gui_to_thread_rx: &Receiver<InputData>)
 -> Result<(), InputData> where H: Hit + 'static, W: Outline + 'static{
    
    let image_height = input_data.image_height;
    let image_width = input_data.image_width;
    let mut pixel_colors = vec![Color::new(0.0,0.0,0.0); image_height * image_width];

    let cam = Camera::new(input_data.camera_settings);
    if let Some(pixels) = static_data.primitives.outline(&cam) {
        for pixel in pixels {
            let pixel_index = (image_height - pixel[1] - 1) * image_width + pixel[0];
            pixel_colors[pixel_index] = Color::new(1.0, 1.0, 1.0);
        }
    }

    let message = gui_to_thread_rx.try_recv();
    match message {
        Ok(input_data) => return Err(input_data),
        Err(err) => {
            match err {
                TryRecvError::Empty => {}
                TryRecvError::Disconnected => {
                    input_data.run = false;
                    return Err(input_data);
                }
            }
        }
    }

    let output = ImageData{pixel_colors, image_width, image_height, samples: 1};
    if thread_to_gui_tx.send(output).is_err() {
        input_data.run = false;
        return Err(input_data);
    }
    Ok(())  
                
 }

pub fn raytrace<H, W>(mut input_data: InputData, static_data: Arc<StaticData<H, W>>, thread_to_gui_tx: Sender<ImageData>, gui_to_thread_rx: &Receiver<InputData>)
 -> Result<(), InputData> where H: Hit + 'static, W: Outline +'static {

    let image_height = input_data.image_height;
    let image_width = input_data.image_width;
    let mut pixel_colors = vec![Color::new(0.0,0.0,0.0); image_height * image_width];
    let cam = Camera::new(input_data.camera_settings);
    for j in 0..image_height{
        for i in 0..image_width{
                let u = (rand_double(0.0, 1.0) + i as f64)/(image_width as f64 - 1.0);
                let v = (rand_double(0.0, 1.0) + (image_height - j) as f64)/((image_height - 1) as f64);
                let r = cam.get_ray(u,v);
                let pixel_index = (j*image_width + i) as usize;
                pixel_colors[pixel_index] = pixel_colors[pixel_index] + ray_color(&r, static_data.background, &static_data.world, input_data.max_depth);
                let message = gui_to_thread_rx.try_recv();
                match message {
                    Ok(input_data) => return Err(input_data),
                    Err(err) => {
                        match err {
                            TryRecvError::Empty => {}
                            TryRecvError::Disconnected => {
                                input_data.run = false;
                                return Err(input_data);
                            }
                        }
                    }
                }
            }
        }

        let output = ImageData{pixel_colors, image_width, image_height, samples: 1};
        if thread_to_gui_tx.send(output).is_err() {
            input_data.run = false;
            return Err(input_data);
        }
    Ok(())
}