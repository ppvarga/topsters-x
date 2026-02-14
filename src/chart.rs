use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub cover_url: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    pub row_sizes: Vec<usize>,
    pub rows: Vec<Vec<Option<Album>>>
}

pub enum ChartPreset {
    Nine,
    TwentyFive,
    Hundred
}

impl Chart {
    pub fn from_preset(preset: ChartPreset) -> Self {
        let layout = match preset {
            ChartPreset::Nine => vec![3, 3, 3],
            ChartPreset::TwentyFive => vec![5, 5, 5, 5, 5],
            ChartPreset::Hundred => vec![
                5, 5, 6, 6, 6,
                10, 10, 10,
                14, 14, 14
            ],
        };

        Self::from_row_sizes(layout)
    }

    pub fn from_row_sizes(row_sizes: Vec<usize>) -> Self {
        let rows = row_sizes.clone()
            .into_iter()
            .map(|size| vec![None; size])
            .collect();

        Self { row_sizes, rows }
    }
}
