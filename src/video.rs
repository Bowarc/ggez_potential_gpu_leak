pub struct Frame {
    inner: ggez::graphics::Image,
}

pub struct Video {
    data: Vec<(Vec<u8>, (usize, usize))>,
    image: ggez::graphics::Image,
    current: usize,
}

impl Video {
    pub fn from_path(
        gfx: &mut ggez::graphics::GraphicsContext,
        path: String,
    ) -> ggez::GameResult<Self> {
        let data = decode(gfx, path);

        println!("Collected {} frames", data.len());

        Ok(Self {
            data,
            image: ggez::graphics::Image::from_color(gfx, 1920, 1080, None),
            current: 0,
        })
    }

    pub fn draw(
        &mut self,
        gfx: &mut ggez::graphics::GraphicsContext,
        canvas: &mut ggez::graphics::Canvas,
    ) -> ggez::GameResult {
        let (data, size) = if let Some((data, size)) = self.data.get(self.current) {
            (data, size)
        } else {
            (&self.data.last().unwrap().0, &self.data.last().unwrap().1)
        };
        self.image = ggez::graphics::Image::from_pixels(
            gfx,
            data,
            wgpu::TextureFormat::Rgba8Unorm,
            size.0 as u32,
            size.1 as u32,
        );

        let desired_size = (1920.0, 1080.0);
        canvas.draw(
            &self.image,
            ggez::graphics::DrawParam::default().scale(ggez::mint::Vector2 {
                x: desired_size.0 / size.0 as f32,
                y: desired_size.1 / size.1 as f32,
            }),
        );
        self.current += 1;
        Ok(())
    }
}

impl Frame {
    pub fn new(
        gfx: &mut ggez::graphics::GraphicsContext,
        data: &[u8],
        size: (usize, usize),
    ) -> Self {
        Self {
            inner: ggez::graphics::Image::from_pixels(
                gfx,
                data,
                wgpu::TextureFormat::Rgba8Unorm,
                size.0 as u32,
                size.1 as u32,
            ),
        }
    }
}

pub fn decode(
    gfx: &mut ggez::graphics::GraphicsContext,
    path: String,
) -> Vec<(Vec<u8>, (usize, usize))> {
    let mut output = Vec::new();

    let mut ictx = crate::ffmpeg::format::input(&path).unwrap();

    let input = ictx
        .streams()
        .best(crate::ffmpeg::media::Type::Video)
        .ok_or(crate::ffmpeg::Error::StreamNotFound)
        .unwrap();
    let video_stream_index = input.index();

    let context_decoder =
        crate::ffmpeg::codec::context::Context::from_parameters(input.parameters()).unwrap();
    let mut decoder = context_decoder.decoder().video().unwrap();

    let mut scaler = crate::ffmpeg::software::scaling::context::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        crate::ffmpeg::format::Pixel::RGBA,
        decoder.width(),
        decoder.height(),
        crate::ffmpeg::software::scaling::flag::Flags::BILINEAR,
    )
    .unwrap();

    let size = (decoder.width() as usize, decoder.height() as usize);

    let mut frame_index = 0;

    let mut receive_and_process_decoded_frames =
        |decoder: &mut crate::ffmpeg::decoder::Video| -> Result<(), crate::ffmpeg::Error> {
            let mut decoded = crate::ffmpeg::util::frame::video::Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = crate::ffmpeg::util::frame::video::Video::empty();
                scaler.run(&decoded, &mut rgb_frame)?;
                // println!("new frame: {:?}", rgb_frame.data(0));
                output.push((rgb_frame.data(0).to_vec(), size));
                // save_file(&rgb_frame, frame_index).unwrap();
                frame_index += 1;
            }
            Ok(())
        };

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet).unwrap();
            receive_and_process_decoded_frames(&mut decoder).unwrap();
        }
    }
    decoder.send_eof().unwrap();
    receive_and_process_decoded_frames(&mut decoder).unwrap();

    output
}
