#![allow(clippy::comparison_chain)]
use crate::{
    pipeline::{self, PipelineBaton, PipelineResult},
    Sharp,
};
use std::path::Path;

impl Sharp {
    pub fn to_icon<P: AsRef<Path>>(mut self, file_out: P) -> Result<Self, String> {
        let mut buffers = Vec::new();

        if self.options.join.is_empty() {
            let result = to_png_buffer(self.options).unwrap();
            self.options = result.baton;
            buffers.push(result.buffer);
        } else {
            let mut final_option = self.options.clone();
            for inp in &self.options.join {
                let mut options = self.options.clone();
                options.join = Vec::new();
                options.input = inp.clone();
                let result = to_png_buffer(options).unwrap();
                final_option = result.baton;
                buffers.push(result.buffer);
            }
            self.options = final_option;
        }

        let icon = encode(buffers)?;

        std::fs::write(file_out.as_ref(), icon).map_err(|e| e.to_string())?;

        Ok(self)
    }
}

fn to_png_buffer(mut options: PipelineBaton) -> Result<PipelineResult, String> {
    options.file_out = String::new();
    options.format_out = "png".to_string();
    pipeline::pipline(options).map_err(|e| e.to_string()).map_err(|e| e.to_string())
}

fn encode(image_buffers: Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut icon_dir: Vec<u8> = Vec::with_capacity(6);
    let mut image_offset = 0u32;
    icon_dir.extend_from_slice(&0u16.to_le_bytes());
    icon_dir.extend_from_slice(&1u16.to_le_bytes());
    icon_dir.extend_from_slice(&(image_buffers.len() as u16).to_le_bytes());
    image_offset += 6;
    buffer.extend(icon_dir);

    // Write our directory entries
    for data in &image_buffers {
        let buf = write_icon_entry(data)?;
        image_offset += 16;
        buffer.extend(buf);
    }

    // Write our icon data
    for (i, image_data) in image_buffers.iter().enumerate() {
        let offset_offset = 6 + (16 * i) + 12;
        if buffer.len() < offset_offset + 4 {
            buffer.resize(offset_offset + 4, 0);
        }
        buffer[offset_offset..offset_offset + 4].copy_from_slice(&image_offset.to_le_bytes());

        if image_data[0] == 0x89 && image_data[1] == 0x50 && image_data[2] == 0x4E && image_data[3] == 0x47 {
            buffer.extend(image_data);
            image_offset += image_data.len() as u32;
        } else {
            buffer.push(image_data[14]);
            image_offset += (image_data.len() - 14) as u32;
        }
    }

    Ok(buffer)
}

fn write_icon_entry(image_data: &[u8]) -> Result<Vec<u8>, String> {
    let mut buffer: Vec<u8> = Vec::with_capacity(16);
    if image_data[12] != 73 && image_data[13] != 72 && image_data[14] != 68 && image_data[15] != 82 {
        return Err("PNG's first chunk must be an IHDR".to_string());
    }
    let mut width = u32::from_be_bytes(image_data[16..20].try_into().unwrap());
    println!("width:{:?}", width);
    let mut height = u32::from_be_bytes(image_data[20..24].try_into().unwrap());
    let bits_per_pixel = image_data[24];
    let color_type = image_data[25];
    let mut color_entries = 0u8;

    if color_type == 3 {
        if image_data[29].to_string() != "P" && image_data[30].to_string() != "L" && image_data[31].to_string() != "T" && image_data[32].to_string() != "E" {
            return Err("PNG's second chunk must be a PLTE if indexed".to_string());
        }
        color_entries = (u32::from_be_bytes(image_data[25..29].try_into().unwrap()) / 3) as _;
    }

    // Do some validation
    if width > 256 {
        return Err("PNG width must not exceed 256".to_string());
    } else if width == 256 {
        width = 0
    }
    if height > 256 {
        return Err("PNG height must not exceed 256".to_string());
    } else if height == 256 {
        height = 0
    }
    // Write width and height
    buffer.push(width as u8);
    buffer.push(height as u8);
    // Write color palettes
    buffer.push(color_entries);
    // Write reserved
    buffer.push(0u8);
    // Write color planes
    buffer.extend_from_slice(&1u16.to_le_bytes());
    // Write bbp
    buffer.extend_from_slice(&(bits_per_pixel as u16).to_le_bytes());
    // Write image data size
    buffer.extend_from_slice(&(image_data.len() as u32).to_le_bytes());

    Ok(buffer)
}
