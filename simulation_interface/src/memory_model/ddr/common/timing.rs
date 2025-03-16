use serde::Deserialize;

// Common parameters across all DDR versions
#[derive(Deserialize)]
pub struct Timing {
    pub rate: u16,
    pub freq: u16,
    pub t_ck: f32,
    pub bl: u16,
    pub cl: u16,
    pub rcd: u16,
    pub rp: u16,
    pub ras: u16,
    pub rc: u16,
    pub wr: u16,
    pub rtp: u16,
    pub cwl: u16,
    pub rtrs: u16,

    pub xp: u16,
    pub ckesr: u16,
    pub cke: u16,
}
