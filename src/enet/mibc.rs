#[doc = "Register `MIBC` reader"]
pub struct R(crate::R<MIBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIBC` writer"]
pub struct W(crate::W<MIBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIBC_SPEC>;
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
impl From<crate::W<MIBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIB_CLEAR` reader - MIB Clear"]
pub type MIB_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `MIB_CLEAR` writer - MIB Clear"]
pub type MIB_CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIBC_SPEC, bool, O>;
#[doc = "Field `MIB_IDLE` reader - MIB Idle"]
pub type MIB_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `MIB_DIS` reader - Disable MIB Logic"]
pub type MIB_DIS_R = crate::BitReader<bool>;
#[doc = "Field `MIB_DIS` writer - Disable MIB Logic"]
pub type MIB_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIBC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 29 - MIB Clear"]
    #[inline(always)]
    pub fn mib_clear(&self) -> MIB_CLEAR_R {
        MIB_CLEAR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MIB Idle"]
    #[inline(always)]
    pub fn mib_idle(&self) -> MIB_IDLE_R {
        MIB_IDLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable MIB Logic"]
    #[inline(always)]
    pub fn mib_dis(&self) -> MIB_DIS_R {
        MIB_DIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - MIB Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mib_clear(&mut self) -> MIB_CLEAR_W<29> {
        MIB_CLEAR_W::new(self)
    }
    #[doc = "Bit 31 - Disable MIB Logic"]
    #[inline(always)]
    #[must_use]
    pub fn mib_dis(&mut self) -> MIB_DIS_W<31> {
        MIB_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIB Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mibc](index.html) module"]
pub struct MIBC_SPEC;
impl crate::RegisterSpec for MIBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mibc::R](R) reader structure"]
impl crate::Readable for MIBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mibc::W](W) writer structure"]
impl crate::Writable for MIBC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIBC to value 0xc000_0000"]
impl crate::Resettable for MIBC_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
