#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]

use crate::{
    input::{CreateRaw, SharpOptions},
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

    pub fn from_icon_file<P: AsRef<Path>>(icon_file: P) -> Result<Self, String> {
        let buffer = std::fs::read(icon_file.as_ref()).map_err(|e| e.to_string())?;
        let entries = decode(buffer)?;
        Self::from_icon(entries)
    }

    pub fn from_icon_buffer(buffer: Vec<u8>) -> Result<Self, String> {
        let entries = decode(buffer)?;
        Self::from_icon(entries)
    }

    fn from_icon(entries: Vec<IconEntry>) -> Result<Self, String> {
        if entries.is_empty() {
            return Err("Not icon data found".to_string());
        }

        let buffers: Vec<Vec<u8>> = entries.iter().map(|entry| entry.image_data.clone()).collect();

        let first = entries.first().unwrap();
        if first.image_type == "png" {
            if buffers.len() > 1 {
                Sharp::new_from_buffers(buffers)
            } else {
                Sharp::new_from_buffer(buffers.first().unwrap().to_vec())
            }
        } else {
            if buffers.len() > 1 {
                Sharp::new_from_buffers_with_opts(
                    buffers,
                    SharpOptions {
                        raw: Some(CreateRaw {
                            width: first.width as _,
                            height: first.height as _,
                            channels: 4,
                            premultiplied: false,
                        }),
                        ..Default::default()
                    },
                )
            } else {
                Sharp::new_from_buffer_with_opts(
                    buffers.first().unwrap().to_vec(),
                    SharpOptions {
                        raw: Some(CreateRaw {
                            width: first.width as _,
                            height: first.height as _,
                            channels: 4,
                            premultiplied: false,
                        }),
                        ..Default::default()
                    },
                )
            }
        }
    }
}

#[derive(Debug, Default)]
struct IconEntry {
    width: u16,
    height: u16,
    colors: u8,
    color_planes: u8,
    bits_per_pixel: u8,
    horizontal_host_spot: u8,
    vertical_host_spot: u8,
    image_size: u8,
    image_offset: u8,
    image_data: Vec<u8>,
    image_type: String,
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

fn decode(buffer: Vec<u8>) -> Result<Vec<IconEntry>, String> {
    let mut offset = 0;

    let buf = read_u16_le(buffer.as_slice(), offset);
    offset += 2;
    if buf != 0 {
        return Err("Reserved must be 0".to_string());
    }
    let buf = read_u16_le(buffer.as_slice(), offset);
    offset += 2;
    let type_ = if buf == 1 || buf == 2 {
        buf
    } else {
        return Err("image type must be ICO or CUR".to_string());
    };
    let buf = read_u16_le(buffer.as_slice(), offset);
    offset += 2;
    let image_count = buf;

    let mut icon_entries = Vec::new();
    for _ in 0..image_count {
        let mut entry = read_icon_entry(buffer.as_slice(), &mut offset, type_)?;
        read_icon_data(buffer.as_slice(), &mut entry);
        icon_entries.push(entry);
    }
    Ok(icon_entries)
}

fn read_u16_le(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap())
}

fn read_u32_be(data: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap())
}

fn read_u32_le(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap())
}

fn read_icon_entry(buffer: &[u8], offset: &mut usize, type_: u16) -> Result<IconEntry, String> {
    let mut entry = IconEntry::default();

    // Read width
    let buf = buffer[*offset];
    *offset += 1;
    if buf == 0 {
        entry.width = 256;
    } else {
        entry.width = buf as _;
    }
    // Read height
    let buf = buffer[*offset];
    *offset += 1;
    if buf == 0 {
        entry.height = 256;
    } else {
        entry.height = buf as _;
    }
    // Read number of colors
    let buf = buffer[*offset];
    *offset += 1;
    entry.colors = buf;
    // Read reserved.
    let buf = buffer[*offset];
    *offset += 1;
    if buf != 0 {
        return Err("Reserved must be 0".to_string());
    }
    // Read color planes or horizontal hotspot
    let buf = read_u16_le(buffer, *offset) as u8;
    *offset += 2;
    if type_ == 1 {
        if buf != 0 && buf != 1 {
            // throw `Color plane was ${buf}, should be 0 or 1`
        }
        entry.color_planes = buf;
    } else if type_ == 2 {
        entry.horizontal_host_spot = buf
    }
    // Read bits per pixel or vertical hotspot
    let buf = read_u16_le(buffer, *offset) as u8;
    *offset += 2;
    if type_ == 1 {
        entry.bits_per_pixel = buf;
    } else if type_ == 2 {
        entry.vertical_host_spot = buf;
    }
    // Read the size of the image data.
    let buf = read_u32_le(buffer, *offset) as u8;
    *offset += 4;
    entry.image_size = buf;
    // Read the offset of the image data.
    let buf = read_u32_le(buffer, *offset) as u8;
    *offset += 4;
    entry.image_offset = buf;

    Ok(entry)
}

fn read_icon_data(buffer: &[u8], icon: &mut IconEntry) {
    let to_off = (icon.image_offset + icon.image_size) as usize;
    let from_off = icon.image_offset as usize;
    let mut image_data = buffer[from_off..to_off].to_vec();
    if image_data[0] == 0x89 && image_data[1] == 0x50 && image_data[2] == 0x4E && image_data[3] == 0x47 {
        icon.image_data = image_data.clone();
        icon.image_type = "png".to_string();
    } else {
        icon.image_type = "bmp".to_string();
        // Get the info header size
        let header_size = read_u32_le(&image_data, 0);
        // Overwrite width/height with ICO defined (GIMP stored a 16x16 BMP in an ICO as 16x32... for some reason)
        // TODO: This is a bit beyond the scope I wanted to stay at, but the reason for above is due to the use of a mask that defines transparency/clipping. For now we"re doing the bogus manual shortening but in the future it would be best to implement the XOR and whatnot bitmap features.

        image_data.extend_from_slice(&(icon.width as u32).to_le_bytes());
        image_data.extend_from_slice(&(icon.height as u32).to_le_bytes());

        let bits_per_pixel = read_u16_le(image_data.as_slice(), 14) as u32;
        // Check if we have BI_BITFIELDS (increases bitmap data offset by 12)
        let has_bit_fields = read_u32_le(image_data.as_slice(), 16) == 3;
        // Get the count of palettes
        let mut palette_entries = read_u32_le(image_data.as_slice(), 32);
        if palette_entries == 0 && bits_per_pixel != 32 {
            palette_entries = 2u32.pow(bits_per_pixel);
        }
        // Get the paletteColorSize -- BITMAPCOREHEADER is 3 bytes, otherwise 4
        let palette_color_size = if header_size == 12 {
            3
        } else {
            4
        };
        let color_table_offset = if has_bit_fields {
            header_size + 12
        } else {
            header_size
        };
        let color_table_size = palette_entries * palette_color_size;
        // Find the starting address of the pixel data.
        let pixel_data_offset = color_table_offset + color_table_size;
        // Build our bitmap header.
        let mut bitmap_header = Vec::with_capacity(14);
        // Write BM header field.
        bitmap_header.push(0x42);
        bitmap_header.push(0x4D);
        // Write file size
        bitmap_header.extend_from_slice(&(icon.image_size as u32 + 14u32).to_le_bytes());

        // Write pixel data offset.
        bitmap_header.extend_from_slice(&(pixel_data_offset + 14).to_le_bytes());

        bitmap_header.extend(image_data);
        icon.image_data = bitmap_header;
    }
}

fn write_icon_entry(image_data: &[u8]) -> Result<Vec<u8>, String> {
    if image_data[0] == 0x89 && image_data[1] == 0x50 && image_data[2] == 0x4E && image_data[3] == 0x47 {
        write_icon_entry_png(image_data)
    } else {
        write_icon_entry_bmp(image_data)
    }
}

fn write_icon_entry_png(image_data: &[u8]) -> Result<Vec<u8>, String> {
    let mut buffer: Vec<u8> = Vec::with_capacity(16);
    if image_data[12] != 73 && image_data[13] != 72 && image_data[14] != 68 && image_data[15] != 82 {
        return Err("PNG's first chunk must be an IHDR".to_string());
    }
    let mut width = read_u32_be(image_data, 16);
    let mut height = read_u32_be(image_data, 20);
    let bits_per_pixel = image_data[24];
    let color_type = image_data[25];
    let mut color_entries = 0u8;

    if color_type == 3 {
        if image_data[29].to_string() != "P" && image_data[30].to_string() != "L" && image_data[31].to_string() != "T" && image_data[32].to_string() != "E" {
            return Err("PNG's second chunk must be a PLTE if indexed".to_string());
        }
        color_entries = (read_u32_be(image_data, 25) / 3) as _;
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

fn write_icon_entry_bmp(image_data: &[u8]) -> Result<Vec<u8>, String> {
    let mut buffer = Vec::with_capacity(16);
    // Get information
    let mut width = read_u32_le(image_data, 18);
    let mut height = read_u32_le(image_data, 22);

    let color_planes = read_u16_le(image_data, 26);
    let mut color_entries = read_u32_le(image_data, 46);
    let bits_per_pixel = read_u16_le(image_data, 28);

    // Do some validation
    if width > 256 {
        return Err("BMP width must not exceed 256".to_string());
    } else if width == 256 {
        width = 0;
    }
    if height > 256 {
        return Err("BMP height must not exceed 256".to_string());
    } else if height == 256 {
        height = 0;
    }

    if color_planes != 1 {
        return Err("BMP color planes must be 1".to_string());
    }

    if color_entries == 0 && bits_per_pixel != 32 {
        color_entries = 2u32.pow(bits_per_pixel as u32);
    }
    if color_entries > 256 {
        color_entries = 0;
    } else if color_entries == 256 {
        color_entries = 255;
    }

    // Write width and height
    buffer.push(width as u8);
    buffer.push(height as u8);
    // Write color palettes
    buffer.push(color_entries as u8);
    // Write reserved
    buffer.push(0u8);
    // Write color planes
    buffer.extend_from_slice(&color_planes.to_le_bytes());
    // Write bbp
    buffer.extend_from_slice(&bits_per_pixel.to_le_bytes());
    // Write image data size
    buffer.extend_from_slice(&(image_data.len() as u32 - 14u32).to_le_bytes());

    Ok(buffer)
}
