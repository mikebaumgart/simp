use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
    time::Duration,
};

use glium::glutin::window::CursorIcon;
use image::{Delay, DynamicImage, Frame, ImageBuffer, Rgba};

pub mod extensions;

#[macro_export]
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

#[macro_export]
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

#[derive(Clone, Debug)]
pub struct Image {
    pub image: DynamicImage,
    pub delay: Duration,
}

impl Image {
    #[inline]
    pub fn new(image: DynamicImage) -> Self {
        Image {
            image,
            delay: Duration::default(),
        }
    }

    #[inline]
    pub fn with_delay(image: DynamicImage, delay: Duration) -> Self {
        Image { image, delay }
    }

    #[inline]
    pub fn buffer(&self) -> &DynamicImage {
        &self.image
    }

    #[inline]
    pub fn buffer_mut(&mut self) -> &mut DynamicImage {
        &mut self.image
    }
}

impl From<ImageBuffer<Rgba<u8>, Vec<u8>>> for Image {
    #[inline]
    fn from(buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Self {
        Image {
            image: DynamicImage::ImageRgba8(buffer),
            delay: Duration::default(),
        }
    }
}

impl From<Frame> for Image {
    #[inline]
    fn from(frame: Frame) -> Self {
        let (num, deno) = frame.delay().numer_denom_ms();
        let delay = Duration::from_millis((num / deno) as u64);
        let buffer = frame.into_buffer();
        Image {
            image: DynamicImage::ImageRgba8(buffer),
            delay,
        }
    }
}

impl From<Image> for Frame {
    #[inline]
    fn from(image: Image) -> Frame {
        let duration = image.delay;
        let frame = image.image.to_rgba8();
        Frame::from_parts(frame, 0, 0, Delay::from_saturating_duration(duration))
    }
}

pub enum UserEvent {
    ImageLoaded(Arc<RwLock<Vec<Image>>>, Option<PathBuf>),
    Resize(Option<Vec<Image>>),
    LoadError(String, PathBuf),
    ErrorMessage(String),
    SetCursor(CursorIcon),
    Save(PathBuf),
    Exit,
}