#[repr(C)]
/// Basic timers
pub struct Tim {
    /// control register 1
    pub cr1: Cr1,
    /// control register 2
    pub cr2: Cr2,
    reserved0: [u8; 4usize],
    /// DMA/Interrupt enable register
    pub dier: Dier,
    /// status register
    pub sr: Sr,
    /// event generation register
    pub egr: Egr,
    reserved1: [u8; 12usize],
    /// counter
    pub cnt: Cnt,
    /// prescaler
    pub psc: Psc,
    /// auto-reload register
    pub arr: Arr,
}

pub struct Cr1 {
    register: ::volatile_register::RW<u32>,
}

impl Cr1 {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut Cr1W) -> &mut Cr1W
    {
        let mut rw = Cr1W { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> Cr1R {
        Cr1R { bits: self.register.read() }
    }
    pub fn write(&mut self, value: Cr1W) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct Cr1R {
    bits: u32,
}

impl Cr1R {
    /// Counter enable
    pub fn cen(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Update disable
    pub fn udis(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Update request source
    pub fn urs(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// One-pulse mode
    pub fn opm(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Auto-reload preload enable
    pub fn arpe(&self) -> bool {
        const OFFSET: u8 = 7;
        self.bits & (1 << OFFSET) != 0
    }
    /// UIF status bit remapping
    pub fn uifremap(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct Cr1W {
    bits: u32,
}

impl Cr1W {
    /// Reset value
    pub fn reset_value() -> Self {
        Cr1W { bits: 0 }
    }
    /// Counter enable
    pub fn cen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Update disable
    pub fn udis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Update request source
    pub fn urs(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// One-pulse mode
    pub fn opm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Auto-reload preload enable
    pub fn arpe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// UIF status bit remapping
    pub fn uifremap(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Cr2 {
    register: ::volatile_register::RW<u32>,
}

impl Cr2 {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut Cr2W) -> &mut Cr2W
    {
        let mut rw = Cr2W { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> Cr2R {
        Cr2R { bits: self.register.read() }
    }
    pub fn write(&mut self, value: Cr2W) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct Cr2R {
    bits: u32,
}

impl Cr2R {
    /// Master mode selection
    pub fn mms(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
pub struct Cr2W {
    bits: u32,
}

impl Cr2W {
    /// Reset value
    pub fn reset_value() -> Self {
        Cr2W { bits: 0 }
    }
    /// Master mode selection
    pub fn mms(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

pub struct Dier {
    register: ::volatile_register::RW<u32>,
}

impl Dier {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut DierW) -> &mut DierW
    {
        let mut rw = DierW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> DierR {
        DierR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: DierW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct DierR {
    bits: u32,
}

impl DierR {
    /// Update DMA request enable
    pub fn ude(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Update interrupt enable
    pub fn uie(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct DierW {
    bits: u32,
}

impl DierW {
    /// Reset value
    pub fn reset_value() -> Self {
        DierW { bits: 0 }
    }
    /// Update DMA request enable
    pub fn ude(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Update interrupt enable
    pub fn uie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Sr {
    register: ::volatile_register::RW<u32>,
}

impl Sr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut SrW) -> &mut SrW
    {
        let mut rw = SrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> SrR {
        SrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: SrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct SrR {
    bits: u32,
}

impl SrR {
    /// Update interrupt flag
    pub fn uif(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct SrW {
    bits: u32,
}

impl SrW {
    /// Reset value
    pub fn reset_value() -> Self {
        SrW { bits: 0 }
    }
    /// Update interrupt flag
    pub fn uif(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Egr {
    register: ::volatile_register::WO<u32>,
}

impl Egr {
    pub fn write(&self, value: EgrW) {
        self.register.write(value.bits);
    }
}

#[derive(Clone, Copy)]
pub struct EgrR {
    bits: u32,
}

impl EgrR {
    /// Update generation
    pub fn ug(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct EgrW {
    bits: u32,
}

impl EgrW {
    /// Reset value
    pub fn reset_value() -> Self {
        EgrW { bits: 0 }
    }
    /// Update generation
    pub fn ug(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Cnt {
    register: ::volatile_register::RW<u32>,
}

impl Cnt {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut CntW) -> &mut CntW
    {
        let mut rw = CntW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> CntR {
        CntR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: CntW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct CntR {
    bits: u32,
}

impl CntR {
    /// Low counter value
    pub fn cnt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    /// UIF Copy
    pub fn uifcpy(&self) -> bool {
        const OFFSET: u8 = 31;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct CntW {
    bits: u32,
}

impl CntW {
    /// Reset value
    pub fn reset_value() -> Self {
        CntW { bits: 0 }
    }
    /// Low counter value
    pub fn cnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

pub struct Psc {
    register: ::volatile_register::RW<u32>,
}

impl Psc {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut PscW) -> &mut PscW
    {
        let mut rw = PscW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> PscR {
        PscR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: PscW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct PscR {
    bits: u32,
}

impl PscR {
    /// Prescaler value
    pub fn psc(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

#[derive(Clone, Copy)]
pub struct PscW {
    bits: u32,
}

impl PscW {
    /// Reset value
    pub fn reset_value() -> Self {
        PscW { bits: 0 }
    }
    /// Prescaler value
    pub fn psc(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

pub struct Arr {
    register: ::volatile_register::RW<u32>,
}

impl Arr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut ArrW) -> &mut ArrW
    {
        let mut rw = ArrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> ArrR {
        ArrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: ArrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct ArrR {
    bits: u32,
}

impl ArrR {
    /// Low Auto-reload value
    pub fn arr(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

#[derive(Clone, Copy)]
pub struct ArrW {
    bits: u32,
}

impl ArrW {
    /// Reset value
    pub fn reset_value() -> Self {
        ArrW { bits: 0 }
    }
    /// Low Auto-reload value
    pub fn arr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
