#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAMUAP` reader - SRAM_U arbitration priority"]
pub type SRAMUAP_R = crate::FieldReader<u8, SRAMUAP_A>;
#[doc = "SRAM_U arbitration priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRAMUAP_A {
    #[doc = "0: Round robin"]
    _00 = 0,
    #[doc = "1: Special round robin (favors SRAM backoor accesses over the processor)"]
    _01 = 1,
    #[doc = "2: Fixed priority. Processor has highest, backdoor has lowest"]
    _10 = 2,
    #[doc = "3: Fixed priority. Backdoor has highest, processor has lowest"]
    _11 = 3,
}
impl From<SRAMUAP_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMUAP_A) -> Self {
        variant as _
    }
}
impl SRAMUAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMUAP_A {
        match self.bits {
            0 => SRAMUAP_A::_00,
            1 => SRAMUAP_A::_01,
            2 => SRAMUAP_A::_10,
            3 => SRAMUAP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SRAMUAP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SRAMUAP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SRAMUAP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SRAMUAP_A::_11
    }
}
#[doc = "Field `SRAMUAP` writer - SRAM_U arbitration priority"]
pub type SRAMUAP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, SRAMUAP_A, 2, O>;
impl<'a, const O: u8> SRAMUAP_W<'a, O> {
    #[doc = "Round robin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SRAMUAP_A::_00)
    }
    #[doc = "Special round robin (favors SRAM backoor accesses over the processor)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SRAMUAP_A::_01)
    }
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SRAMUAP_A::_10)
    }
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SRAMUAP_A::_11)
    }
}
#[doc = "Field `SRAMUWP` reader - SRAM_U write protect"]
pub type SRAMUWP_R = crate::BitReader<bool>;
#[doc = "Field `SRAMUWP` writer - SRAM_U write protect"]
pub type SRAMUWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SRAMLAP` reader - SRAM_L arbitration priority"]
pub type SRAMLAP_R = crate::FieldReader<u8, SRAMLAP_A>;
#[doc = "SRAM_L arbitration priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRAMLAP_A {
    #[doc = "0: Round robin"]
    _00 = 0,
    #[doc = "1: Special round robin (favors SRAM backoor accesses over the processor)"]
    _01 = 1,
    #[doc = "2: Fixed priority. Processor has highest, backdoor has lowest"]
    _10 = 2,
    #[doc = "3: Fixed priority. Backdoor has highest, processor has lowest"]
    _11 = 3,
}
impl From<SRAMLAP_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMLAP_A) -> Self {
        variant as _
    }
}
impl SRAMLAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMLAP_A {
        match self.bits {
            0 => SRAMLAP_A::_00,
            1 => SRAMLAP_A::_01,
            2 => SRAMLAP_A::_10,
            3 => SRAMLAP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SRAMLAP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SRAMLAP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SRAMLAP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SRAMLAP_A::_11
    }
}
#[doc = "Field `SRAMLAP` writer - SRAM_L arbitration priority"]
pub type SRAMLAP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, SRAMLAP_A, 2, O>;
impl<'a, const O: u8> SRAMLAP_W<'a, O> {
    #[doc = "Round robin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SRAMLAP_A::_00)
    }
    #[doc = "Special round robin (favors SRAM backoor accesses over the processor)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SRAMLAP_A::_01)
    }
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SRAMLAP_A::_10)
    }
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SRAMLAP_A::_11)
    }
}
#[doc = "Field `SRAMLWP` reader - SRAM_L Write Protect"]
pub type SRAMLWP_R = crate::BitReader<bool>;
#[doc = "Field `SRAMLWP` writer - SRAM_L Write Protect"]
pub type SRAMLWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 24:25 - SRAM_U arbitration priority"]
    #[inline(always)]
    pub fn sramuap(&self) -> SRAMUAP_R {
        SRAMUAP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - SRAM_U write protect"]
    #[inline(always)]
    pub fn sramuwp(&self) -> SRAMUWP_R {
        SRAMUWP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:29 - SRAM_L arbitration priority"]
    #[inline(always)]
    pub fn sramlap(&self) -> SRAMLAP_R {
        SRAMLAP_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - SRAM_L Write Protect"]
    #[inline(always)]
    pub fn sramlwp(&self) -> SRAMLWP_R {
        SRAMLWP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25 - SRAM_U arbitration priority"]
    #[inline(always)]
    #[must_use]
    pub fn sramuap(&mut self) -> SRAMUAP_W<24> {
        SRAMUAP_W::new(self)
    }
    #[doc = "Bit 26 - SRAM_U write protect"]
    #[inline(always)]
    #[must_use]
    pub fn sramuwp(&mut self) -> SRAMUWP_W<26> {
        SRAMUWP_W::new(self)
    }
    #[doc = "Bits 28:29 - SRAM_L arbitration priority"]
    #[inline(always)]
    #[must_use]
    pub fn sramlap(&mut self) -> SRAMLAP_W<28> {
        SRAMLAP_W::new(self)
    }
    #[doc = "Bit 30 - SRAM_L Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn sramlwp(&mut self) -> SRAMLWP_W<30> {
        SRAMLWP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
