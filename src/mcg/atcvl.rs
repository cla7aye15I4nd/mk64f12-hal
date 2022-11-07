#[doc = "Register `ATCVL` reader"]
pub struct R(crate::R<ATCVL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATCVL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATCVL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATCVL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATCVL` writer"]
pub struct W(crate::W<ATCVL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATCVL_SPEC>;
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
impl From<crate::W<ATCVL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATCVL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATCVL` reader - ATM Compare Value Low"]
pub type ATCVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATCVL` writer - ATM Compare Value Low"]
pub type ATCVL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ATCVL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ATM Compare Value Low"]
    #[inline(always)]
    pub fn atcvl(&self) -> ATCVL_R {
        ATCVL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - ATM Compare Value Low"]
    #[inline(always)]
    #[must_use]
    pub fn atcvl(&mut self) -> ATCVL_W<0> {
        ATCVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Auto Trim Compare Value Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atcvl](index.html) module"]
pub struct ATCVL_SPEC;
impl crate::RegisterSpec for ATCVL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [atcvl::R](R) reader structure"]
impl crate::Readable for ATCVL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atcvl::W](W) writer structure"]
impl crate::Writable for ATCVL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATCVL to value 0"]
impl crate::Resettable for ATCVL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
