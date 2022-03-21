use graphics::{image, Context};
use opengl_graphics::{GlGraphics, Texture};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapMetaData {
    pub tile_width: f64,
    pub tile_height: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapData {
    pub meta: MapMetaData,
    pub tiles: Vec<Tile>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Tile {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

pub struct Map {
    pub meta_data: MapMetaData,
    pub tiles: Vec<Tile>,
    pub map_tex: Texture,
}

impl Map {
    pub fn load(path: &str) -> Self {
        use opengl_graphics::{Filter, TextureSettings};
        use std::{fs::File, path::PathBuf};

        let path = PathBuf::from(path);
        let tex_path = path.with_extension("png");
        let json_path = path.with_extension("json");

        let map_data: MapData = serde_json::from_reader(File::open(json_path).unwrap()).unwrap();

        Self {
            meta_data: map_data.meta,
            tiles: map_data.tiles,
            map_tex: Texture::from_path(&tex_path, &TextureSettings::new().filter(Filter::Nearest))
                .unwrap(),
        }
    }

    pub fn render(&self, c: Context, gl: &mut GlGraphics) {
        image(&self.map_tex, c.transform, gl);
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Map")
            .field("meta_data", &self.meta_data)
            .field("tiles", &self.tiles)
            .field("map_tex id", &self.map_tex.get_id())
            .finish()
    }
}
