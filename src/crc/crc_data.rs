#[doc = "Register `DATA` reader"]
pub struct R(crate::R<CRC_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA` writer"]
pub struct W(crate::W<CRC_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_DATA_SPEC>;
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
impl From<crate::W<CRC_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LL` reader - CRC Low Lower Byte"]
pub type LL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LL` writer - CRC Low Lower Byte"]
pub type LL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRC_DATA_SPEC, u8, u8, 8, O>;
#[doc = "Field `LU` reader - CRC Low Upper Byte"]
pub type LU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LU` writer - CRC Low Upper Byte"]
pub type LU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRC_DATA_SPEC, u8, u8, 8, O>;
#[doc = "Field `HL` reader - CRC High Lower Byte"]
pub type HL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HL` writer - CRC High Lower Byte"]
pub type HL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRC_DATA_SPEC, u8, u8, 8, O>;
#[doc = "Field `HU` reader - CRC High Upper Byte"]
pub type HU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HU` writer - CRC High Upper Byte"]
pub type HU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRC_DATA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - CRC Low Lower Byte"]
    #[inline(always)]
    pub fn ll(&self) -> LL_R {
        LL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CRC Low Upper Byte"]
    #[inline(always)]
    pub fn lu(&self) -> LU_R {
        LU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CRC High Lower Byte"]
    #[inline(always)]
    pub fn hl(&self) -> HL_R {
        HL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CRC High Upper Byte"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRC Low Lower Byte"]
    #[inline(always)]
    #[must_use]
    pub fn ll(&mut self) -> LL_W<0> {
        LL_W::new(self)
    }
    #[doc = "Bits 8:15 - CRC Low Upper Byte"]
    #[inline(always)]
    #[must_use]
    pub fn lu(&mut self) -> LU_W<8> {
        LU_W::new(self)
    }
    #[doc = "Bits 16:23 - CRC High Lower Byte"]
    #[inline(always)]
    #[must_use]
    pub fn hl(&mut self) -> HL_W<16> {
        HL_W::new(self)
    }
    #[doc = "Bits 24:31 - CRC High Upper Byte"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HU_W<24> {
        HU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_data](index.html) module"]
pub struct CRC_DATA_SPEC;
impl crate::RegisterSpec for CRC_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_data::R](R) reader structure"]
impl crate::Readable for CRC_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_data::W](W) writer structure"]
impl crate::Writable for CRC_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA to value 0xffff_ffff"]
impl crate::Resettable for CRC_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
