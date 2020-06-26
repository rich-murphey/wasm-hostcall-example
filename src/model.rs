#[derive(Debug, Serialize, Deserialize)]
pub struct AB {
    pub a: u32,
    pub b: String,
}

const CD_N :usize = 32;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct CD {
    pub c: i32,
    pub d: ArrayString::<[u8; CD_N]>,
}

impl CD {
    #[allow(unused)]
    pub fn from(c: i32, d: &str) -> Self {
        Self {
            c,
            d: ArrayString::<[u8; CD_N]>::from(d).unwrap()
        }
    }
}
