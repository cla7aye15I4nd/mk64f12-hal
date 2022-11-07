#[doc = "Register `PCR6` reader"]
pub struct R(crate::R<PCR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCR6` writer"]
pub struct W(crate::W<PCR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PCR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PS` reader - Pull Select"]
pub type PS_R = crate::BitReader<PS_A>;
#[doc = "Pull Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PS_A {
    #[doc = "0: Internal pulldown resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    _0 = 0,
    #[doc = "1: Internal pullup resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    _1 = 1,
}
impl From<PS_A> for bool {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as u8 != 0
    }
}
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            false => PS_A::_0,
            true => PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PS_A::_1
    }
}
#[doc = "Field `PS` writer - Pull Select"]
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR6_SPEC, PS_A, O>;
impl<'a, const O: u8> PS_W<'a, O> {
    #[doc = "Internal pulldown resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PS_A::_0)
    }
    #[doc = "Internal pullup resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PS_A::_1)
    }
}
#[doc = "Field `PE` reader - Pull Enable"]
pub type PE_R = crate::BitReader<PE_A>;
#[doc = "Pull Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    #[doc = "0: Internal pullup or pulldown resistor is not enabled on the corresponding pin."]
    _0 = 0,
    #[doc = "1: Internal pullup or pulldown resistor is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::_0,
            true => PE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PE_A::_1
    }
}
#[doc = "Field `PE` writer - Pull Enable"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR6_SPEC, PE_A, O>;
impl<'a, const O: u8> PE_W<'a, O> {
    #[doc = "Internal pullup or pulldown resistor is not enabled on the corresponding pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PE_A::_0)
    }
    #[doc = "Internal pullup or pulldown resistor is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PE_A::_1)
    }
}
#[doc = "Field `SRE` reader - Slew Rate Enable"]
pub type SRE_R = crate::BitReader<SRE_A>;
#[doc = "Slew Rate Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRE_A {
    #[doc = "0: Fast slew rate is configured on the corresponding pin, if the pin is configured as a digital output."]
    _0 = 0,
    #[doc = "1: Slow slew rate is configured on the corresponding pin, if the pin is configured as a digital output."]
    _1 = 1,
}
impl From<SRE_A> for bool {
    #[inline(always)]
    fn from(variant: SRE_A) -> Self {
        variant as u8 != 0
    }
}
impl SRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRE_A {
        match self.bits {
            false => SRE_A::_0,
            true => SRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRE_A::_1
    }
}
#[doc = "Field `SRE` writer - Slew Rate Enable"]
pub type SRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR6_SPEC, SRE_A, O>;
impl<'a, const O: u8> SRE_W<'a, O> {
    #[doc = "Fast slew rate is configured on the corresponding pin, if the pin is configured as a digital output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRE_A::_0)
    }
    #[doc = "Slow slew rate is configured on the corresponding pin, if the pin is configured as a digital output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRE_A::_1)
    }
}
#[doc = "Field `PFE` reader - Passive Filter Enable"]
pub type PFE_R = crate::BitReader<PFE_A>;
#[doc = "Passive Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFE_A {
    #[doc = "0: Passive input filter is disabled on the corresponding pin."]
    _0 = 0,
    #[doc = "1: Passive input filter is enabled on the corresponding pin, if the pin is configured as a digital input. Refer to the device data sheet for filter characteristics."]
    _1 = 1,
}
impl From<PFE_A> for bool {
    #[inline(always)]
    fn from(variant: PFE_A) -> Self {
        variant as u8 != 0
    }
}
impl PFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFE_A {
        match self.bits {
            false => PFE_A::_0,
            true => PFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFE_A::_1
    }
}
#[doc = "Field `PFE` writer - Passive Filter Enable"]
pub type PFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR6_SPEC, PFE_A, O>;
impl<'a, const O: u8> PFE_W<'a, O> {
    #[doc = "Passive input filter is disabled on the corresponding pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PFE_A::_0)
    }
    #[doc = "Passive input filter is enabled on the corresponding pin, if the pin is configured as a digital input. Refer to the device data sheet for filter characteristics."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PFE_A::_1)
    }
}
#[doc = "Field `ODE` reader - Open Drain Enable"]
pub type ODE_R = crate::BitReader<ODE_A>;
#[doc = "Open Drain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODE_A {
    #[doc = "0: Open drain output is disabled on the corresponding pin."]
    _0 = 0,
    #[doc = "1: Open drain output is enabled on the corresponding pin, if the pin is configured as a digital output."]
    _1 = 1,
}
impl From<ODE_A> for bool {
    #[inline(always)]
    fn from(variant: ODE_A) -> Self {
        variant as u8 != 0
    }
}
impl ODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODE_A {
        match self.bits {
            false => ODE_A::_0,
            true => ODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ODE_A::_1
    }
}
#[doc = "Field `ODE` writer - Open Drain Enable"]
pub type ODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR6_SPEC, ODE_A, O>;
impl<'a, const O: u8> ODE_W<'a, O> {
    #[doc = "Open drain output is disabled on the corresponding pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ODE_A::_0)
    }
    #[doc = "Open drain output is enabled on the corresponding pin, if the pin is configured as a digital output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ODE_A::_1)
    }
}
#[doc = "Field `DSE` reader - Drive Strength Enable"]
pub type DSE_R = crate::BitReader<DSE_A>;
#[doc = "Drive Strength Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSE_A {
    #[doc = "0: Low drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    _0 = 0,
    #[doc = "1: High drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    _1 = 1,
}
impl From<DSE_A> for bool {
    #[inline(always)]
    fn from(variant: DSE_A) -> Self {
        variant as u8 != 0
    }
}
impl DSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSE_A {
        match self.bits {
            false => DSE_A::_0,
            true => DSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSE_A::_1
    }
}
#[doc = "Field `DSE` writer - Drive Strength Enable"]
pub type DSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR6_SPEC, DSE_A, O>;
impl<'a, const O: u8> DSE_W<'a, O> {
    #[doc = "Low drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSE_A::_0)
    }
    #[doc = "High drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSE_A::_1)
    }
}
#[doc = "Field `MUX` reader - Pin Mux Control"]
pub type MUX_R = crate::FieldReader<u8, MUX_A>;
#[doc = "Pin Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUX_A {
    #[doc = "0: Pin disabled (analog)."]
    _000 = 0,
    #[doc = "1: Alternative 1 (GPIO)."]
    _001 = 1,
    #[doc = "2: Alternative 2 (chip-specific)."]
    _010 = 2,
    #[doc = "3: Alternative 3 (chip-specific)."]
    _011 = 3,
    #[doc = "4: Alternative 4 (chip-specific)."]
    _100 = 4,
    #[doc = "5: Alternative 5 (chip-specific)."]
    _101 = 5,
    #[doc = "6: Alternative 6 (chip-specific)."]
    _110 = 6,
    #[doc = "7: Alternative 7 (chip-specific)."]
    _111 = 7,
}
impl From<MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_A) -> Self {
        variant as _
    }
}
impl MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_A {
        match self.bits {
            0 => MUX_A::_000,
            1 => MUX_A::_001,
            2 => MUX_A::_010,
            3 => MUX_A::_011,
            4 => MUX_A::_100,
            5 => MUX_A::_101,
            6 => MUX_A::_110,
            7 => MUX_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == MUX_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == MUX_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == MUX_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == MUX_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == MUX_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == MUX_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == MUX_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == MUX_A::_111
    }
}
#[doc = "Field `MUX` writer - Pin Mux Control"]
pub type MUX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PCR6_SPEC, u8, MUX_A, 3, O>;
impl<'a, const O: u8> MUX_W<'a, O> {
    #[doc = "Pin disabled (analog)."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(MUX_A::_000)
    }
    #[doc = "Alternative 1 (GPIO)."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(MUX_A::_001)
    }
    #[doc = "Alternative 2 (chip-specific)."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(MUX_A::_010)
    }
    #[doc = "Alternative 3 (chip-specific)."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(MUX_A::_011)
    }
    #[doc = "Alternative 4 (chip-specific)."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(MUX_A::_100)
    }
    #[doc = "Alternative 5 (chip-specific)."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(MUX_A::_101)
    }
    #[doc = "Alternative 6 (chip-specific)."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(MUX_A::_110)
    }
    #[doc = "Alternative 7 (chip-specific)."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(MUX_A::_111)
    }
}
#[doc = "Field `LK` reader - Lock Register"]
pub type LK_R = crate::BitReader<LK_A>;
#[doc = "Lock Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LK_A {
    #[doc = "0: Pin Control Register fields \\[15:0\\]
are not locked."]
    _0 = 0,
    #[doc = "1: Pin Control Register fields \\[15:0\\]
are locked and cannot be updated until the next system reset."]
    _1 = 1,
}
impl From<LK_A> for bool {
    #[inline(always)]
    fn from(variant: LK_A) -> Self {
        variant as u8 != 0
    }
}
impl LK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LK_A {
        match self.bits {
            false => LK_A::_0,
            true => LK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LK_A::_1
    }
}
#[doc = "Field `LK` writer - Lock Register"]
pub type LK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR6_SPEC, LK_A, O>;
impl<'a, const O: u8> LK_W<'a, O> {
    #[doc = "Pin Control Register fields \\[15:0\\]
are not locked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LK_A::_0)
    }
    #[doc = "Pin Control Register fields \\[15:0\\]
are locked and cannot be updated until the next system reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LK_A::_1)
    }
}
#[doc = "Field `IRQC` reader - Interrupt Configuration"]
pub type IRQC_R = crate::FieldReader<u8, IRQC_A>;
#[doc = "Interrupt Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IRQC_A {
    #[doc = "0: Interrupt/DMA request disabled."]
    _0000 = 0,
    #[doc = "1: DMA request on rising edge."]
    _0001 = 1,
    #[doc = "2: DMA request on falling edge."]
    _0010 = 2,
    #[doc = "3: DMA request on either edge."]
    _0011 = 3,
    #[doc = "8: Interrupt when logic 0."]
    _1000 = 8,
    #[doc = "9: Interrupt on rising-edge."]
    _1001 = 9,
    #[doc = "10: Interrupt on falling-edge."]
    _1010 = 10,
    #[doc = "11: Interrupt on either edge."]
    _1011 = 11,
    #[doc = "12: Interrupt when logic 1."]
    _1100 = 12,
}
impl From<IRQC_A> for u8 {
    #[inline(always)]
    fn from(variant: IRQC_A) -> Self {
        variant as _
    }
}
impl IRQC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IRQC_A> {
        match self.bits {
            0 => Some(IRQC_A::_0000),
            1 => Some(IRQC_A::_0001),
            2 => Some(IRQC_A::_0010),
            3 => Some(IRQC_A::_0011),
            8 => Some(IRQC_A::_1000),
            9 => Some(IRQC_A::_1001),
            10 => Some(IRQC_A::_1010),
            11 => Some(IRQC_A::_1011),
            12 => Some(IRQC_A::_1100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == IRQC_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == IRQC_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == IRQC_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == IRQC_A::_0011
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == IRQC_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == IRQC_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == IRQC_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == IRQC_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == IRQC_A::_1100
    }
}
#[doc = "Field `IRQC` writer - Interrupt Configuration"]
pub type IRQC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCR6_SPEC, u8, IRQC_A, 4, O>;
impl<'a, const O: u8> IRQC_W<'a, O> {
    #[doc = "Interrupt/DMA request disabled."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(IRQC_A::_0000)
    }
    #[doc = "DMA request on rising edge."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(IRQC_A::_0001)
    }
    #[doc = "DMA request on falling edge."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(IRQC_A::_0010)
    }
    #[doc = "DMA request on either edge."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(IRQC_A::_0011)
    }
    #[doc = "Interrupt when logic 0."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(IRQC_A::_1000)
    }
    #[doc = "Interrupt on rising-edge."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(IRQC_A::_1001)
    }
    #[doc = "Interrupt on falling-edge."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(IRQC_A::_1010)
    }
    #[doc = "Interrupt on either edge."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(IRQC_A::_1011)
    }
    #[doc = "Interrupt when logic 1."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(IRQC_A::_1100)
    }
}
#[doc = "Field `ISF` reader - Interrupt Status Flag"]
pub type ISF_R = crate::BitReader<ISF_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF_A> for bool {
    #[inline(always)]
    fn from(variant: ISF_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF_A {
        match self.bits {
            false => ISF_A::_0,
            true => ISF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF_A::_1
    }
}
#[doc = "Field `ISF` writer - Interrupt Status Flag"]
pub type ISF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR6_SPEC, ISF_A, O>;
impl<'a, const O: u8> ISF_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Pull Select"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pull Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slew Rate Enable"]
    #[inline(always)]
    pub fn sre(&self) -> SRE_R {
        SRE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Passive Filter Enable"]
    #[inline(always)]
    pub fn pfe(&self) -> PFE_R {
        PFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Open Drain Enable"]
    #[inline(always)]
    pub fn ode(&self) -> ODE_R {
        ODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Drive Strength Enable"]
    #[inline(always)]
    pub fn dse(&self) -> DSE_R {
        DSE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Pin Mux Control"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Lock Register"]
    #[inline(always)]
    pub fn lk(&self) -> LK_R {
        LK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Interrupt Configuration"]
    #[inline(always)]
    pub fn irqc(&self) -> IRQC_R {
        IRQC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pull Select"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<0> {
        PS_W::new(self)
    }
    #[doc = "Bit 1 - Pull Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<1> {
        PE_W::new(self)
    }
    #[doc = "Bit 2 - Slew Rate Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sre(&mut self) -> SRE_W<2> {
        SRE_W::new(self)
    }
    #[doc = "Bit 4 - Passive Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfe(&mut self) -> PFE_W<4> {
        PFE_W::new(self)
    }
    #[doc = "Bit 5 - Open Drain Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ode(&mut self) -> ODE_W<5> {
        ODE_W::new(self)
    }
    #[doc = "Bit 6 - Drive Strength Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dse(&mut self) -> DSE_W<6> {
        DSE_W::new(self)
    }
    #[doc = "Bits 8:10 - Pin Mux Control"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MUX_W<8> {
        MUX_W::new(self)
    }
    #[doc = "Bit 15 - Lock Register"]
    #[inline(always)]
    #[must_use]
    pub fn lk(&mut self) -> LK_W<15> {
        LK_W::new(self)
    }
    #[doc = "Bits 16:19 - Interrupt Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn irqc(&mut self) -> IRQC_W<16> {
        IRQC_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf(&mut self) -> ISF_W<24> {
        ISF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Control Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr6](index.html) module"]
pub struct PCR6_SPEC;
impl crate::RegisterSpec for PCR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcr6::R](R) reader structure"]
impl crate::Readable for PCR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr6::W](W) writer structure"]
impl crate::Writable for PCR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCR6 to value 0"]
impl crate::Resettable for PCR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
