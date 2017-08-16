use std::sync::{Arc, Mutex};

use vulkano::image::immutable::ImmutableImage;
use vulkano::sampler::Filter;
use vulkano::sampler::SamplerAddressMode;
use vulkano::sampler::MipmapMode;
use vulkano::device::Device;
use vulkano::device::Queue;
use vulkano::sampler::Sampler;
use vulkano::image::Dimensions::Dim2d;
use vulkano::format::FormatDesc;
use vulkano::format::AcceptsPixels;
use vulkano;

use image;

use core::engine_settings;

/// A simple format enum used to describe image formats supported by Ori
/// Currently has no effect because the only "always" supported type is 8Bit RGBA
#[derive(Eq, PartialEq)]
pub enum TextureColorFormats {
    R8G8B8A8_SRGB,
    R8G8B8A8_UNORM,
    R8_GREYSCALE,
}


pub struct TextureBuilder {
    //sampler
    //Sampling information if the image is larger or smaller than the original
    mag_filter: Filter,
    min_filter: Filter,
    //defines mipmapping mode
    mip_map_mode: MipmapMode,
    //defines how vulkano should handle U-V-W coordinates outside of 0.0-1.0
    address_u: SamplerAddressMode,
    address_v: SamplerAddressMode,
    address_w: SamplerAddressMode,

    // adds to the mip_mapping distance
    mip_lod_bias: f32,
    //set the filtering of this texture, this should usually be read from the settings
    max_anisotropy: f32,
    //Sets the max and min mipmapping level to use
    min_lod: f32,
    max_lod: f32,

    //image
    //color format
    color_format: TextureColorFormats,
    //Some helpful postprogressing
    b_blur: bool,
    blur_factor: f32,

    b_unsharpen: bool,
    sharp_factor: f32,
    sharp_threshold: i32,

    b_brighten: bool,
    brighten_factor: i32,

    b_flipv: bool,
    b_fliph: bool,

    b_rotate90: bool,
    b_rotate180: bool,
    b_rotate270: bool,

    //Create info (this won't be included in the final texture)
    image_path: String,
    device: Arc<Device>,
    queue: Arc<Queue>,
    engine_settings: Arc<Mutex<engine_settings::EngineSettings>>,
}


impl TextureBuilder {
    ///Creates a new builder struct with default parameters from an image at `image_path`
    pub fn from_image(
        image_path: &str,
        device: Arc<Device>,
        queue: Arc<Queue>,
        engine_settings: Arc<Mutex<engine_settings::EngineSettings>>
    ) -> Self{
        //Create the default builder
        TextureBuilder{
            //sampler
            //Sampling information if the image is larger or smaller than the original
            mag_filter: Filter::Linear,
            min_filter: Filter::Linear,
            //defines mipmapping mode
            mip_map_mode: MipmapMode::Nearest,
            //defines how vulkano should handle U-V-W coordinates outside of 0.0-1.0
            address_u: SamplerAddressMode::Repeat,
            address_v: SamplerAddressMode::Repeat,
            address_w: SamplerAddressMode::Repeat,

            // adds to the mip_mapping distance
            mip_lod_bias: 0.0,
            //set the filtering of this texture, this should usually be read from the settings
            max_anisotropy: 1.0,
            //Sets the max and min mipmapping level to use
            min_lod: 0.0,
            max_lod: 0.0,

            //image
            //color format
            color_format: TextureColorFormats::R8G8B8A8_SRGB,
            //Some helpful postprogressing
            b_blur: false,
            blur_factor: 0.0,

            b_unsharpen: false,
            sharp_factor: 0.0,
            sharp_threshold: 0,

            b_brighten: false,
            brighten_factor: 0,

            b_flipv: false,
            b_fliph: false,

            b_rotate90: false,
            b_rotate180: false,
            b_rotate270: false,

            //Create info (this won't be included in the final texture)
            image_path: String::from(image_path),
            device: device,
            queue: queue,
            engine_settings: engine_settings,
        }
    }

    ///Sets new filtering technic for the sampler
    pub fn with_sampling_filter(mut self, new_filter: Filter) -> Self{
        self.mag_filter = new_filter.clone();
        self.min_filter = new_filter.clone();
        self
    }

    ///Sets new mipmapping mode for the sampler
    pub fn with_mip_map_moe(mut self, new_mode: MipmapMode) -> Self{
        self.mip_map_mode = new_mode;
        self
    }

    ///Sets new tiling mode for the sampler
    pub fn with_tiling_mode(mut self, new_mode: SamplerAddressMode) -> Self{
        self.address_u = new_mode.clone();
        self.address_v = new_mode.clone();
        self.address_w = new_mode.clone();
        self
    }
    ///Sets new mip lod bias for the sampler
    pub fn with_lod_bias(mut self, bias: f32) -> Self{
        self.mip_lod_bias = bias;
        self
    }

    ///Sets new max anisotropic level for the sampler
    ///#panic This will panic if max < 1.0
    pub fn wit_max_anisotropy(mut self, max: f32) -> Self{
        //have to test that it is => 1.0 otherwise this will create a runtime error
        if max < 1.0 {
            panic!("The anisotropic max has to be equal ot larger than 1.0");
        }
        self.max_anisotropy = max;
        self
    }

    ///Sets new min and max mip map level for the sampler
    ///#panic this will panic if min is greater than max
    pub fn with_min_and_max_mip_level(mut self, min: f32, max: f32) -> Self{
        //test min and max
        if min > max {
            panic!("the min mip map level has to be equal or smaller than the max level");
        }

        self.min_lod = min;
        self.max_lod = max;
        self
    }

    ///Sets new target color format for the image
    ///The imported image will be converted to this after importing
    pub fn with_format(mut self, format: TextureColorFormats) -> Self{
        self.color_format = format;
        self
    }

    ///The imported image will be blured by `factor` after importing
    pub fn with_blur(mut self, factor: f32) -> Self{
        self.b_blur = true;
        self.blur_factor = factor;
        self
    }

    ///The imported image get a unsharpen masked applied with the blur of `factor` and the sharpening of `thresold` after importing
    pub fn with_unsharpening(mut self, factor: f32, thresold: i32) -> Self{
        self.b_unsharpen = true;
        self.sharp_factor = factor;
        self.sharp_threshold = thresold;
        self
    }

    ///The imported image will be brightened by `factor` after importing (tip the value can be)
    ///negative to darken the image
    pub fn with_brightening(mut self, factor: i32) -> Self{
        self.b_brighten = true;
        self.brighten_factor = factor;
        self
    }

    ///this will flip the image vertically
    pub fn with_flipped_v(mut self) -> Self{
        self.b_flipv = true;
        self
    }
    ///this will flip the image horizontally
    pub fn with_flipped_h(mut self) -> Self{
        self.b_fliph = true;
        self
    }
    ///this will rotate the image 90 degree
    pub fn with_rotation_90(mut self) -> Self{
        self.b_rotate90 = true;
        self
    }
    ///this will rotate the image 180 degree
    pub fn with_rotation_180(mut self) -> Self{
        self.b_rotate180 = true;
        self
    }
    ///this will rotate the image 270 degree
    pub fn with_rotation_270(mut self) -> Self{
        self.b_rotate270 = true;
        self
    }

    ///This function will use the information currently present in the `TextureBuilder`
    ///and create a `core::resources::Texture` from it
    pub fn build_with_name(mut self, texture_name: &str) -> Arc<Texture>
        {
        //Setup a sampler from the info
        let tmp_sampler = Sampler::new(
            self.device.clone(),
            self.mag_filter,
            self.min_filter,
            self.mip_map_mode,
            self.address_u,
            self.address_v,
            self.address_w,
            self.mip_lod_bias,
            self.max_anisotropy,
            self.min_lod,
            self.max_lod,
        ).expect("Failed to generate albedo sampler");

        //Now load a the texture
        let texture = {
            let (texture_tmp, tex_future) = {
                let mut image = image::open(&self.image_path.to_string())
                .expect("failed to load png normal in creation");

                //now apply, based on the settings all the post progressing
                //after applying everything we can convert the Dynamic image into the correct format

                //blur
                if self.b_blur {
                    image = image.blur(self.blur_factor);
                }
                //unsharpening
                if self.b_unsharpen {
                    image = image.unsharpen(self.sharp_factor, self.sharp_threshold);
                }
                //brighten
                if self.b_brighten {
                    image = image.brighten(self.brighten_factor);
                }
                //flipping
                if self.b_flipv{
                    image = image.flipv();
                }
                if self.b_fliph {
                    image = image.fliph();
                }
                //rotation 90-270 degree
                if self.b_rotate90 {
                    image = image.rotate90();
                }
                if self.b_rotate180 {
                    image = image.rotate180();
                }
                if self.b_rotate270 {
                    image = image.rotate270();
                }

                /* TODO Implement a parmaeter F: format for dynamic image formats
                //if specified load as greyscale
                if self.color_format == TextureColorFormats::R8_GREYSCALE{
                    let final_image = image.to_luma();
                    //Now transform the image::* into a vulkano image
                    let (width, height) = final_image.dimensions();
                    let image_data = final_image.into_raw().clone();

                    //return image and its GpuFuture
                    ImmutableImage::from_iter(
                        image_data.iter().cloned(),
                        Dim2d { width: width, height: height },
                        //Set format dependent on self.color_format
                        vulkano::format::R8Srgb,
                        Some(self.queue.family()),
                        self.queue.clone()).expect("failed to create immutable image")
                //else load the image as 8bit rgba srgb
                }else{
                    let final_image = image.to_rgba();
                    //Now transform the image::* into a vulkano image
                    let (width, height) = final_image.dimensions();
                    let image_data = final_image.into_raw().clone();

                    //return image and its GpuFuture
                    ImmutableImage::from_iter(
                        image_data.iter().cloned(),
                        Dim2d { width: width, height: height },
                        //Set format dependent on self.color_format
                        vulkano::format::R8G8B8A8Srgb,
                        Some(self.queue.family()),
                        self.queue.clone()).expect("failed to create immutable image")
                }
                */

                //NOTE the static image format is not final
                let final_image = image.to_rgba();
                //Now transform the image::* into a vulkano image
                let (width, height) = final_image.dimensions();
                let image_data = final_image.into_raw().clone();

                //return image and its GpuFuture
                ImmutableImage::from_iter(
                    image_data.iter().cloned(),
                    Dim2d { width: width, height: height },
                    //Set format dependent on self.color_format
                    vulkano::format::R8G8B8A8Srgb,
                    Some(self.queue.family()),
                    self.queue.clone()).expect("failed to create immutable image")
            };
            //drop the future to wait for gpu
            texture_tmp
        };

        let texture_struct = Texture{
            name: String::from(texture_name),
            texture: texture,
            sampler: tmp_sampler,
            original_path: self.image_path.clone(),
        };

        Arc::new(texture_struct)
    }
}
/*
let sampler_albedo_tmp = vulkano::sampler::Sampler::new(
    device.clone(),
    vulkano::sampler::Filter::Linear,
    vulkano::sampler::Filter::Linear,
    vulkano::sampler::MipmapMode::Nearest,
    vulkano::sampler::SamplerAddressMode::Repeat,
    vulkano::sampler::SamplerAddressMode::Repeat,
    vulkano::sampler::SamplerAddressMode::Repeat,
    0.0, 1.0, 0.0, 0.0
).expect("Failed to generate albedo sampler");
*/

///The Texture holds a images as well as the sampler, mipmapping etc for this texture is stored
/// withing the `vulkano::image::immutable::ImmutableImage`.
///Several textures can be compined in a material
#[derive(Clone)]
pub struct Texture {
    ///A name which can be used to reference the texture
    pub name: String,
    texture: Arc<ImmutableImage<vulkano::format::R8G8B8A8Srgb>>,
    sampler: Arc<vulkano::sampler::Sampler>,

    original_path: String,
}

///The implementation doesn't change anything on this texture
impl Texture{
    ///Returns the raw `Arc<ImmutableImage<vulkano::format::R8G8B8A8Srgb>>`
    pub fn get_raw_texture(&self) -> Arc<ImmutableImage<vulkano::format::R8G8B8A8Srgb>>{
        self.texture.clone()
    }

    ///Returns the raw `Arc<vulkano::sampler::Sampler>`
    pub fn get_raw_sampler(&self) -> Arc<vulkano::sampler::Sampler>{
        self.sampler.clone()
    }
}
