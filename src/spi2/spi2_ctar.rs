#[doc = "Register `CTAR%s` reader"]
pub struct R(crate::R<SPI2_CTAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI2_CTAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI2_CTAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI2_CTAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTAR%s` writer"]
pub struct W(crate::W<SPI2_CTAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI2_CTAR_SPEC>;
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
impl From<crate::W<SPI2_CTAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI2_CTAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BR` reader - Baud Rate Scaler"]
pub type BR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BR` writer - Baud Rate Scaler"]
pub type BR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI2_CTAR_SPEC, u8, u8, 4, O>;
#[doc = "Field `DT` reader - Delay After Transfer Scaler"]
pub type DT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT` writer - Delay After Transfer Scaler"]
pub type DT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI2_CTAR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ASC` reader - After SCK Delay Scaler"]
pub type ASC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ASC` writer - After SCK Delay Scaler"]
pub type ASC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI2_CTAR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CSSCK` reader - PCS to SCK Delay Scaler"]
pub type CSSCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSSCK` writer - PCS to SCK Delay Scaler"]
pub type CSSCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI2_CTAR_SPEC, u8, u8, 4, O>;
#[doc = "Field `PBR` reader - Baud Rate Prescaler"]
pub type PBR_R = crate::FieldReader<u8, PBR_A>;
#[doc = "Baud Rate Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PBR_A {
    #[doc = "0: Baud Rate Prescaler value is 2."]
    _00 = 0,
    #[doc = "1: Baud Rate Prescaler value is 3."]
    _01 = 1,
    #[doc = "2: Baud Rate Prescaler value is 5."]
    _10 = 2,
    #[doc = "3: Baud Rate Prescaler value is 7."]
    _11 = 3,
}
impl From<PBR_A> for u8 {
    #[inline(always)]
    fn from(variant: PBR_A) -> Self {
        variant as _
    }
}
impl PBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBR_A {
        match self.bits {
            0 => PBR_A::_00,
            1 => PBR_A::_01,
            2 => PBR_A::_10,
            3 => PBR_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PBR_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PBR_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PBR_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PBR_A::_11
    }
}
#[doc = "Field `PBR` writer - Baud Rate Prescaler"]
pub type PBR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SPI2_CTAR_SPEC, u8, PBR_A, 2, O>;
impl<'a, const O: u8> PBR_W<'a, O> {
    #[doc = "Baud Rate Prescaler value is 2."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PBR_A::_00)
    }
    #[doc = "Baud Rate Prescaler value is 3."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PBR_A::_01)
    }
    #[doc = "Baud Rate Prescaler value is 5."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PBR_A::_10)
    }
    #[doc = "Baud Rate Prescaler value is 7."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PBR_A::_11)
    }
}
#[doc = "Field `PDT` reader - Delay after Transfer Prescaler"]
pub type PDT_R = crate::FieldReader<u8, PDT_A>;
#[doc = "Delay after Transfer Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PDT_A {
    #[doc = "0: Delay after Transfer Prescaler value is 1."]
    _00 = 0,
    #[doc = "1: Delay after Transfer Prescaler value is 3."]
    _01 = 1,
    #[doc = "2: Delay after Transfer Prescaler value is 5."]
    _10 = 2,
    #[doc = "3: Delay after Transfer Prescaler value is 7."]
    _11 = 3,
}
impl From<PDT_A> for u8 {
    #[inline(always)]
    fn from(variant: PDT_A) -> Self {
        variant as _
    }
}
impl PDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDT_A {
        match self.bits {
            0 => PDT_A::_00,
            1 => PDT_A::_01,
            2 => PDT_A::_10,
            3 => PDT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PDT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PDT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PDT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PDT_A::_11
    }
}
#[doc = "Field `PDT` writer - Delay after Transfer Prescaler"]
pub type PDT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SPI2_CTAR_SPEC, u8, PDT_A, 2, O>;
impl<'a, const O: u8> PDT_W<'a, O> {
    #[doc = "Delay after Transfer Prescaler value is 1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PDT_A::_00)
    }
    #[doc = "Delay after Transfer Prescaler value is 3."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PDT_A::_01)
    }
    #[doc = "Delay after Transfer Prescaler value is 5."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PDT_A::_10)
    }
    #[doc = "Delay after Transfer Prescaler value is 7."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PDT_A::_11)
    }
}
#[doc = "Field `PASC` reader - After SCK Delay Prescaler"]
pub type PASC_R = crate::FieldReader<u8, PASC_A>;
#[doc = "After SCK Delay Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PASC_A {
    #[doc = "0: Delay after Transfer Prescaler value is 1."]
    _00 = 0,
    #[doc = "1: Delay after Transfer Prescaler value is 3."]
    _01 = 1,
    #[doc = "2: Delay after Transfer Prescaler value is 5."]
    _10 = 2,
    #[doc = "3: Delay after Transfer Prescaler value is 7."]
    _11 = 3,
}
impl From<PASC_A> for u8 {
    #[inline(always)]
    fn from(variant: PASC_A) -> Self {
        variant as _
    }
}
impl PASC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PASC_A {
        match self.bits {
            0 => PASC_A::_00,
            1 => PASC_A::_01,
            2 => PASC_A::_10,
            3 => PASC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PASC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PASC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PASC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PASC_A::_11
    }
}
#[doc = "Field `PASC` writer - After SCK Delay Prescaler"]
pub type PASC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SPI2_CTAR_SPEC, u8, PASC_A, 2, O>;
impl<'a, const O: u8> PASC_W<'a, O> {
    #[doc = "Delay after Transfer Prescaler value is 1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PASC_A::_00)
    }
    #[doc = "Delay after Transfer Prescaler value is 3."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PASC_A::_01)
    }
    #[doc = "Delay after Transfer Prescaler value is 5."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PASC_A::_10)
    }
    #[doc = "Delay after Transfer Prescaler value is 7."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PASC_A::_11)
    }
}
#[doc = "Field `PCSSCK` reader - PCS to SCK Delay Prescaler"]
pub type PCSSCK_R = crate::FieldReader<u8, PCSSCK_A>;
#[doc = "PCS to SCK Delay Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCSSCK_A {
    #[doc = "0: PCS to SCK Prescaler value is 1."]
    _00 = 0,
    #[doc = "1: PCS to SCK Prescaler value is 3."]
    _01 = 1,
    #[doc = "2: PCS to SCK Prescaler value is 5."]
    _10 = 2,
    #[doc = "3: PCS to SCK Prescaler value is 7."]
    _11 = 3,
}
impl From<PCSSCK_A> for u8 {
    #[inline(always)]
    fn from(variant: PCSSCK_A) -> Self {
        variant as _
    }
}
impl PCSSCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSSCK_A {
        match self.bits {
            0 => PCSSCK_A::_00,
            1 => PCSSCK_A::_01,
            2 => PCSSCK_A::_10,
            3 => PCSSCK_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PCSSCK_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PCSSCK_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PCSSCK_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PCSSCK_A::_11
    }
}
#[doc = "Field `PCSSCK` writer - PCS to SCK Delay Prescaler"]
pub type PCSSCK_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SPI2_CTAR_SPEC, u8, PCSSCK_A, 2, O>;
impl<'a, const O: u8> PCSSCK_W<'a, O> {
    #[doc = "PCS to SCK Prescaler value is 1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PCSSCK_A::_00)
    }
    #[doc = "PCS to SCK Prescaler value is 3."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PCSSCK_A::_01)
    }
    #[doc = "PCS to SCK Prescaler value is 5."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PCSSCK_A::_10)
    }
    #[doc = "PCS to SCK Prescaler value is 7."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PCSSCK_A::_11)
    }
}
#[doc = "Field `LSBFE` reader - LSB First"]
pub type LSBFE_R = crate::BitReader<LSBFE_A>;
#[doc = "LSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBFE_A {
    #[doc = "0: Data is transferred MSB first."]
    _0 = 0,
    #[doc = "1: Data is transferred LSB first."]
    _1 = 1,
}
impl From<LSBFE_A> for bool {
    #[inline(always)]
    fn from(variant: LSBFE_A) -> Self {
        variant as u8 != 0
    }
}
impl LSBFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSBFE_A {
        match self.bits {
            false => LSBFE_A::_0,
            true => LSBFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LSBFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LSBFE_A::_1
    }
}
#[doc = "Field `LSBFE` writer - LSB First"]
pub type LSBFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2_CTAR_SPEC, LSBFE_A, O>;
impl<'a, const O: u8> LSBFE_W<'a, O> {
    #[doc = "Data is transferred MSB first."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSBFE_A::_0)
    }
    #[doc = "Data is transferred LSB first."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSBFE_A::_1)
    }
}
#[doc = "Field `CPHA` reader - Clock Phase"]
pub type CPHA_R = crate::BitReader<CPHA_A>;
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    #[doc = "0: Data is captured on the leading edge of SCK and changed on the following edge."]
    _0 = 0,
    #[doc = "1: Data is changed on the leading edge of SCK and captured on the following edge."]
    _1 = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::_0,
            true => CPHA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPHA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPHA_A::_1
    }
}
#[doc = "Field `CPHA` writer - Clock Phase"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2_CTAR_SPEC, CPHA_A, O>;
impl<'a, const O: u8> CPHA_W<'a, O> {
    #[doc = "Data is captured on the leading edge of SCK and changed on the following edge."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPHA_A::_0)
    }
    #[doc = "Data is changed on the leading edge of SCK and captured on the following edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPHA_A::_1)
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CPOL_R = crate::BitReader<CPOL_A>;
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    #[doc = "0: The inactive state value of SCK is low."]
    _0 = 0,
    #[doc = "1: The inactive state value of SCK is high."]
    _1 = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::_0,
            true => CPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPOL_A::_1
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2_CTAR_SPEC, CPOL_A, O>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    #[doc = "The inactive state value of SCK is low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOL_A::_0)
    }
    #[doc = "The inactive state value of SCK is high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOL_A::_1)
    }
}
#[doc = "Field `FMSZ` reader - Frame Size"]
pub type FMSZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FMSZ` writer - Frame Size"]
pub type FMSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI2_CTAR_SPEC, u8, u8, 4, O>;
#[doc = "Field `DBR` reader - Double Baud Rate"]
pub type DBR_R = crate::BitReader<DBR_A>;
#[doc = "Double Baud Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBR_A {
    #[doc = "0: The baud rate is computed normally with a 50/50 duty cycle."]
    _0 = 0,
    #[doc = "1: The baud rate is doubled with the duty cycle depending on the Baud Rate Prescaler."]
    _1 = 1,
}
impl From<DBR_A> for bool {
    #[inline(always)]
    fn from(variant: DBR_A) -> Self {
        variant as u8 != 0
    }
}
impl DBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBR_A {
        match self.bits {
            false => DBR_A::_0,
            true => DBR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBR_A::_1
    }
}
#[doc = "Field `DBR` writer - Double Baud Rate"]
pub type DBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2_CTAR_SPEC, DBR_A, O>;
impl<'a, const O: u8> DBR_W<'a, O> {
    #[doc = "The baud rate is computed normally with a 50/50 duty cycle."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBR_A::_0)
    }
    #[doc = "The baud rate is doubled with the duty cycle depending on the Baud Rate Prescaler."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBR_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Baud Rate Scaler"]
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Delay After Transfer Scaler"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - After SCK Delay Scaler"]
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PCS to SCK Delay Scaler"]
    #[inline(always)]
    pub fn cssck(&self) -> CSSCK_R {
        CSSCK_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn pbr(&self) -> PBR_R {
        PBR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Delay after Transfer Prescaler"]
    #[inline(always)]
    pub fn pdt(&self) -> PDT_R {
        PDT_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - After SCK Delay Prescaler"]
    #[inline(always)]
    pub fn pasc(&self) -> PASC_R {
        PASC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PCS to SCK Delay Prescaler"]
    #[inline(always)]
    pub fn pcssck(&self) -> PCSSCK_R {
        PCSSCK_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - LSB First"]
    #[inline(always)]
    pub fn lsbfe(&self) -> LSBFE_R {
        LSBFE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:30 - Frame Size"]
    #[inline(always)]
    pub fn fmsz(&self) -> FMSZ_R {
        FMSZ_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Double Baud Rate"]
    #[inline(always)]
    pub fn dbr(&self) -> DBR_R {
        DBR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Baud Rate Scaler"]
    #[inline(always)]
    #[must_use]
    pub fn br(&mut self) -> BR_W<0> {
        BR_W::new(self)
    }
    #[doc = "Bits 4:7 - Delay After Transfer Scaler"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<4> {
        DT_W::new(self)
    }
    #[doc = "Bits 8:11 - After SCK Delay Scaler"]
    #[inline(always)]
    #[must_use]
    pub fn asc(&mut self) -> ASC_W<8> {
        ASC_W::new(self)
    }
    #[doc = "Bits 12:15 - PCS to SCK Delay Scaler"]
    #[inline(always)]
    #[must_use]
    pub fn cssck(&mut self) -> CSSCK_W<12> {
        CSSCK_W::new(self)
    }
    #[doc = "Bits 16:17 - Baud Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pbr(&mut self) -> PBR_W<16> {
        PBR_W::new(self)
    }
    #[doc = "Bits 18:19 - Delay after Transfer Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pdt(&mut self) -> PDT_W<18> {
        PDT_W::new(self)
    }
    #[doc = "Bits 20:21 - After SCK Delay Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pasc(&mut self) -> PASC_W<20> {
        PASC_W::new(self)
    }
    #[doc = "Bits 22:23 - PCS to SCK Delay Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pcssck(&mut self) -> PCSSCK_W<22> {
        PCSSCK_W::new(self)
    }
    #[doc = "Bit 24 - LSB First"]
    #[inline(always)]
    #[must_use]
    pub fn lsbfe(&mut self) -> LSBFE_W<24> {
        LSBFE_W::new(self)
    }
    #[doc = "Bit 25 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<25> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 26 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<26> {
        CPOL_W::new(self)
    }
    #[doc = "Bits 27:30 - Frame Size"]
    #[inline(always)]
    #[must_use]
    pub fn fmsz(&mut self) -> FMSZ_W<27> {
        FMSZ_W::new(self)
    }
    #[doc = "Bit 31 - Double Baud Rate"]
    #[inline(always)]
    #[must_use]
    pub fn dbr(&mut self) -> DBR_W<31> {
        DBR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock and Transfer Attributes Register (In Master Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2_ctar](index.html) module"]
pub struct SPI2_CTAR_SPEC;
impl crate::RegisterSpec for SPI2_CTAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi2_ctar::R](R) reader structure"]
impl crate::Readable for SPI2_CTAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi2_ctar::W](W) writer structure"]
impl crate::Writable for SPI2_CTAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTAR%s to value 0x7800_0000"]
impl crate::Resettable for SPI2_CTAR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7800_0000;
}
