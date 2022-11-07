#[doc = "Register `WORD011` reader"]
pub struct R(crate::R<WORD011_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WORD011_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WORD011_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WORD011_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WORD011` writer"]
pub struct W(crate::W<WORD011_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WORD011_SPEC>;
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
impl From<crate::W<WORD011_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WORD011_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_BYTE_3` reader - Data byte 3 of Rx/Tx frame."]
pub type DATA_BYTE_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BYTE_3` writer - Data byte 3 of Rx/Tx frame."]
pub type DATA_BYTE_3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WORD011_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA_BYTE_2` reader - Data byte 2 of Rx/Tx frame."]
pub type DATA_BYTE_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BYTE_2` writer - Data byte 2 of Rx/Tx frame."]
pub type DATA_BYTE_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WORD011_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA_BYTE_1` reader - Data byte 1 of Rx/Tx frame."]
pub type DATA_BYTE_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BYTE_1` writer - Data byte 1 of Rx/Tx frame."]
pub type DATA_BYTE_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WORD011_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA_BYTE_0` reader - Data byte 0 of Rx/Tx frame."]
pub type DATA_BYTE_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BYTE_0` writer - Data byte 0 of Rx/Tx frame."]
pub type DATA_BYTE_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WORD011_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_3(&self) -> DATA_BYTE_3_R {
        DATA_BYTE_3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_2(&self) -> DATA_BYTE_2_R {
        DATA_BYTE_2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_1(&self) -> DATA_BYTE_1_R {
        DATA_BYTE_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    pub fn data_byte_0(&self) -> DATA_BYTE_0_R {
        DATA_BYTE_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 3 of Rx/Tx frame."]
    #[inline(always)]
    #[must_use]
    pub fn data_byte_3(&mut self) -> DATA_BYTE_3_W<0> {
        DATA_BYTE_3_W::new(self)
    }
    #[doc = "Bits 8:15 - Data byte 2 of Rx/Tx frame."]
    #[inline(always)]
    #[must_use]
    pub fn data_byte_2(&mut self) -> DATA_BYTE_2_W<8> {
        DATA_BYTE_2_W::new(self)
    }
    #[doc = "Bits 16:23 - Data byte 1 of Rx/Tx frame."]
    #[inline(always)]
    #[must_use]
    pub fn data_byte_1(&mut self) -> DATA_BYTE_1_W<16> {
        DATA_BYTE_1_W::new(self)
    }
    #[doc = "Bits 24:31 - Data byte 0 of Rx/Tx frame."]
    #[inline(always)]
    #[must_use]
    pub fn data_byte_0(&mut self) -> DATA_BYTE_0_W<24> {
        DATA_BYTE_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Buffer 11 WORD0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [word011](index.html) module"]
pub struct WORD011_SPEC;
impl crate::RegisterSpec for WORD011_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [word011::R](R) reader structure"]
impl crate::Readable for WORD011_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [word011::W](W) writer structure"]
impl crate::Writable for WORD011_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WORD011 to value 0"]
impl crate::Resettable for WORD011_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
