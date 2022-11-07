#[doc = "Register `WORD115` reader"]
pub struct R(crate::R<WORD115_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WORD115_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WORD115_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WORD115_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WORD115` writer"]
pub struct W(crate::W<WORD115_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WORD115_SPEC>;
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
impl From<crate::W<WORD115_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WORD115_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_BYTE_7` reader - Data byte 7 of Rx/Tx frame."]
pub type DATA_BYTE_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BYTE_7` writer - Data byte 7 of Rx/Tx frame."]
pub type DATA_BYTE_7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WORD115_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA_BYTE_6` reader - Data byte 6 of Rx/Tx frame."]
pub type DATA_BYTE_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BYTE_6` writer - Data byte 6 of Rx/Tx frame."]
pub type DATA_BYTE_6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WORD115_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA_BYTE_5` reader - Data byte 5 of Rx/Tx frame."]
pub type DATA_BYTE_5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BYTE_5` writer - Data byte 5 of Rx/Tx frame."]
pub type DATA_BYTE_5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WORD115_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA_BYTE_4` reader - Data byte 4 of Rx/Tx frame."]
pub type DATA_BYTE_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BYTE_4` writer - Data byte 4 of Rx/Tx frame."]
pub type DATA_BYTE_4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WORD115_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data byte 7 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_7(&self) -> DATA_BYTE_7_R {
        DATA_BYTE_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 6 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_6(&self) -> DATA_BYTE_6_R {
        DATA_BYTE_6_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 5 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_5(&self) -> DATA_BYTE_5_R {
        DATA_BYTE_5_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 4 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_4(&self) -> DATA_BYTE_4_R {
        DATA_BYTE_4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 7 of Rx/Tx frame."]
    #[inline(always)]
    #[must_use]
    pub fn data_byte_7(&mut self) -> DATA_BYTE_7_W<0> {
        DATA_BYTE_7_W::new(self)
    }
    #[doc = "Bits 8:15 - Data byte 6 of Rx/Tx frame."]
    #[inline(always)]
    #[must_use]
    pub fn data_byte_6(&mut self) -> DATA_BYTE_6_W<8> {
        DATA_BYTE_6_W::new(self)
    }
    #[doc = "Bits 16:23 - Data byte 5 of Rx/Tx frame."]
    #[inline(always)]
    #[must_use]
    pub fn data_byte_5(&mut self) -> DATA_BYTE_5_W<16> {
        DATA_BYTE_5_W::new(self)
    }
    #[doc = "Bits 24:31 - Data byte 4 of Rx/Tx frame."]
    #[inline(always)]
    #[must_use]
    pub fn data_byte_4(&mut self) -> DATA_BYTE_4_W<24> {
        DATA_BYTE_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Buffer 15 WORD1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [word115](index.html) module"]
pub struct WORD115_SPEC;
impl crate::RegisterSpec for WORD115_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [word115::R](R) reader structure"]
impl crate::Readable for WORD115_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [word115::W](W) writer structure"]
impl crate::Writable for WORD115_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WORD115 to value 0"]
impl crate::Resettable for WORD115_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
