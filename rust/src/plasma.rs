use crate::android_bitmap::AndroidBitmapInfo;

const FIXED_BITS: usize = 16;
const ANGLE_BITS: usize = 8;
const PALETTE_BITS: usize = 8;
const OPTIMIZE_WRITES: bool = false;

pub struct Plasma {
    fixed_bits: usize,
    fixed_one: isize,
    fixed_mask: usize,
    angle_bits: usize,
    angle_size: usize,
    angle_mask: usize,
    palette_bits: usize,
    palette_size: usize,
    palette_mask: usize,
    angle_sin_tab: Vec<isize>,
    palette: Vec<u16>,

    pi: isize,
    yt1_incr: isize,
    yt2_incr: isize,
    xt1_incr: isize,
    xt2_incr: isize,
}

impl Plasma {
    pub fn new() -> Self {
        Self::create(FIXED_BITS, ANGLE_BITS, PALETTE_BITS)
    }

    fn create(fixed_bits: usize, angle_bits: usize, palette_bits: usize) -> Self {
        let fixed_one = 1 << fixed_bits;
        let angle_size = (1 << angle_bits) + 1;
        let palette_size = 1 << palette_bits;

        let fixed_mask = (1 << fixed_bits) - 1;
        let angle_mask = (1 << angle_bits) - 1;
        let palette_mask = (1 << palette_bits) - 1;

        let pi = 1 << (angle_bits - 1);

        let from = |x: f64| -> isize { (x * fixed_one as f64) as isize };

        Plasma {
            fixed_bits,
            fixed_one,
            fixed_mask,
            angle_bits,
            angle_mask,
            angle_size,
            palette_bits,
            palette_size,
            palette_mask,
            angle_sin_tab: Self::init_angles(fixed_one, angle_size, pi),
            palette: Self::init_palette(palette_size),
            pi,
            yt1_incr: from(1. / 100.),
            yt2_incr: from(1. / 163.),
            xt1_incr: from(1. / 173.),
            xt2_incr: from(1. / 242.),
        }
    }

    fn init_angles(fixed_one: isize, angle_size: usize, pi: isize) -> Vec<isize> {
        let pi = pi as f64;
        let fixed_one = fixed_one as f64;
        let mut tab: Vec<isize> = vec![0; angle_size];

        for nn in 0..angle_size {
            let radians = nn as f64 * std::f64::consts::PI / pi;
            tab[nn] = (f64::sin(radians) * fixed_one) as isize
        }

        tab
    }

    fn init_palette(palette_size: usize) -> Vec<u16> {
        let mut tab: Vec<u16> = vec![0; palette_size];

        fn make565(red: usize, green: usize, blue: usize) -> u16 {
            (((red << 8) & 0xf800) | ((green << 3) & 0x07e0) | ((blue >> 3) & 0x001f)) as u16
        }

        let nmax = [
            0,
            (palette_size / 4),
            2 * (palette_size / 4),
            3 * (palette_size / 4),
            palette_size,
        ];

        for nn in nmax[0]..nmax[1] {
            let jj = (nn - nmax[0]) * 4 * 255 / palette_size;
            tab[nn] = make565(255, jj, 255 - jj);
        }

        for nn in nmax[1]..nmax[2] {
            let jj = (nn - nmax[1]) * 4 * 255 / palette_size;
            tab[nn] = make565(255 - jj, 255, jj);
        }

        for nn in nmax[2]..nmax[3] {
            let jj = (nn - nmax[2]) * 4 * 255 / palette_size;
            tab[nn] = make565(0, 255 - jj, 255);
        }

        for nn in nmax[3]..nmax[4] {
            let jj = (nn - nmax[3]) * 4 * 255 / palette_size;
            tab[nn] = make565(jj, 0, 255);
        }

        tab
    }
}

impl Plasma {
    #[inline(always)]
    fn from(&self, x: f64) -> isize {
        (x * self.fixed_one as f64) as isize
    }

    #[inline(always)]
    fn angle(&self, x: isize) -> isize {
        x >> (self.fixed_bits - self.angle_bits)
    }

    #[inline(always)]
    fn palette_index(&self, x: isize) -> usize {
        let mut xx = x.abs();
        if xx >= self.fixed_one {
            xx = self.fixed_one - 1;
        }
        (xx as usize >> (self.fixed_bits - self.palette_bits)) & self.palette_mask
    }

    #[inline(always)]
    fn angle_sin(&self, a: isize) -> isize {
        self.angle_sin_tab[a as usize & self.angle_mask]
    }

    #[inline(always)]
    fn sin(&self, f: isize) -> isize {
        self.angle_sin(self.angle(f))
    }

    #[inline(always)]
    fn cos(&self, f: isize) -> isize {
        self.angle_sin(self.angle(f) + self.pi / 2)
    }

    #[inline(always)]
    fn palette(&self, mut x: isize) -> u16 {
        self.palette[self.palette_index(x)]
    }

    pub unsafe fn fill_plasma(
        &self,
        info: &AndroidBitmapInfo,
        pixels: *mut ::std::os::raw::c_void,
        t: i64,
    ) {
        let mut pixels = pixels as *mut u8;

        let t = t as f64;
        let mut yt1 = self.from(t / 1230.);
        let mut yt2 = yt1;
        let mut xt10 = self.from(t / 3000.);
        let mut xt20 = xt10;

        for _ in 0..info.height {
            let mut line = pixels as *mut u16;

            let base = self.sin(yt1) + self.sin(yt2);
            yt1 += self.yt1_incr;
            yt2 += self.yt2_incr;

            let mut xt1 = xt10;
            let mut xt2 = xt20;

            if OPTIMIZE_WRITES {
                let line_end = line.add(info.width as usize);

                if line < line_end {
                    if (line as usize & 3) != 0 {
                        let ii = base + self.sin(xt1) + self.sin(xt2);
                        let pp = self.palette(ii >> 2);
                        xt1 += self.xt1_incr;
                        xt2 += self.xt2_incr;

                        line.write(pp);
                        line = line.add(1);
                    }

                    while line.add(2) < line_end {
                        let i1 = base + self.sin(xt1) + self.sin(xt2);
                        let p1 = self.palette(i1 >> 2);
                        xt1 += self.xt1_incr;
                        xt2 += self.xt2_incr;

                        let i2 = base + self.sin(xt1) + self.sin(xt2);
                        let p2 = self.palette(i2 >> 2);
                        xt1 += self.xt1_incr;
                        xt2 += self.xt2_incr;

                        (line as *mut u32).write((p1 as u32) << 16 | (p2 as u32));
                        line = line.add(2);
                    }

                    if line < line_end {
                        let ii = base + self.sin(xt1) + self.sin(xt2);
                        let pp = self.palette(ii >> 2);
                        line.write(pp);
                        line = line.add(1);
                    }
                }
            } else {
                for xx in 0..info.width {
                    let ii = base + self.sin(xt1) + self.sin(xt2);
                    let pp = self.palette(ii >> 2);

                    xt1 += self.xt1_incr;
                    xt2 += self.xt2_incr;

                    line.add(xx as usize).write(pp);
                }
            }

            pixels = pixels.add(info.stride as usize);
        }
    }
}
