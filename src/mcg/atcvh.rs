#[doc = "Register `ATCVH` reader"]
pub struct R(crate::R<ATCVH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATCVH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATCVH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATCVH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATCVH` writer"]
pub struct W(crate::W<ATCVH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATCVH_SPEC>;
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
impl From<crate::W<ATCVH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATCVH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATCVH` reader - ATM Compare Value High"]
pub type ATCVH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATCVH` writer - ATM Compare Value High"]
pub type ATCVH_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ATCVH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ATM Compare Value High"]
    #[inline(always)]
    pub fn atcvh(&self) -> ATCVH_R {
        ATCVH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - ATM Compare Value High"]
    #[inline(always)]
    #[must_use]
    pub fn atcvh(&mut self) -> ATCVH_W<0> {
        ATCVH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Auto Trim Compare Value High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atcvh](index.html) module"]
pub struct ATCVH_SPEC;
impl crate::RegisterSpec for ATCVH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [atcvh::R](R) reader structure"]
impl crate::Readable for ATCVH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atcvh::W](W) writer structure"]
impl crate::Writable for ATCVH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATCVH to value 0"]
impl crate::Resettable for ATCVH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
