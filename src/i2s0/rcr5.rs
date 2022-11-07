#[doc = "Register `RCR5` reader"]
pub struct R(crate::R<RCR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR5` writer"]
pub struct W(crate::W<RCR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR5_SPEC>;
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
impl From<crate::W<RCR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBT` reader - First Bit Shifted"]
pub type FBT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FBT` writer - First Bit Shifted"]
pub type FBT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCR5_SPEC, u8, u8, 5, O>;
#[doc = "Field `W0W` reader - Word 0 Width"]
pub type W0W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `W0W` writer - Word 0 Width"]
pub type W0W_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCR5_SPEC, u8, u8, 5, O>;
#[doc = "Field `WNW` reader - Word N Width"]
pub type WNW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WNW` writer - Word N Width"]
pub type WNW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCR5_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 8:12 - First Bit Shifted"]
    #[inline(always)]
    pub fn fbt(&self) -> FBT_R {
        FBT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Word 0 Width"]
    #[inline(always)]
    pub fn w0w(&self) -> W0W_R {
        W0W_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Word N Width"]
    #[inline(always)]
    pub fn wnw(&self) -> WNW_R {
        WNW_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - First Bit Shifted"]
    #[inline(always)]
    #[must_use]
    pub fn fbt(&mut self) -> FBT_W<8> {
        FBT_W::new(self)
    }
    #[doc = "Bits 16:20 - Word 0 Width"]
    #[inline(always)]
    #[must_use]
    pub fn w0w(&mut self) -> W0W_W<16> {
        W0W_W::new(self)
    }
    #[doc = "Bits 24:28 - Word N Width"]
    #[inline(always)]
    #[must_use]
    pub fn wnw(&mut self) -> WNW_W<24> {
        WNW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Receive Configuration 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr5](index.html) module"]
pub struct RCR5_SPEC;
impl crate::RegisterSpec for RCR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcr5::R](R) reader structure"]
impl crate::Readable for RCR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr5::W](W) writer structure"]
impl crate::Writable for RCR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCR5 to value 0"]
impl crate::Resettable for RCR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
