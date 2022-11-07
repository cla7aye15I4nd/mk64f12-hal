#[doc = "Register `CLK_RECOVER_CTRL` reader"]
pub struct R(crate::R<CLK_RECOVER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_RECOVER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_RECOVER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_RECOVER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_RECOVER_CTRL` writer"]
pub struct W(crate::W<CLK_RECOVER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_RECOVER_CTRL_SPEC>;
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
impl From<crate::W<CLK_RECOVER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_RECOVER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESTART_IFRTRIM_EN` reader - Restart from IFR trim value"]
pub type RESTART_IFRTRIM_EN_R = crate::BitReader<RESTART_IFRTRIM_EN_A>;
#[doc = "Restart from IFR trim value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESTART_IFRTRIM_EN_A {
    #[doc = "0: Trim fine adjustment always works based on the previous updated trim fine value (default)"]
    _0 = 0,
    #[doc = "1: Trim fine restarts from the IFR trim value whenever bus_reset/bus_resume is detected or module enable is desasserted"]
    _1 = 1,
}
impl From<RESTART_IFRTRIM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RESTART_IFRTRIM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RESTART_IFRTRIM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESTART_IFRTRIM_EN_A {
        match self.bits {
            false => RESTART_IFRTRIM_EN_A::_0,
            true => RESTART_IFRTRIM_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESTART_IFRTRIM_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESTART_IFRTRIM_EN_A::_1
    }
}
#[doc = "Field `RESTART_IFRTRIM_EN` writer - Restart from IFR trim value"]
pub type RESTART_IFRTRIM_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CLK_RECOVER_CTRL_SPEC, RESTART_IFRTRIM_EN_A, O>;
impl<'a, const O: u8> RESTART_IFRTRIM_EN_W<'a, O> {
    #[doc = "Trim fine adjustment always works based on the previous updated trim fine value (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESTART_IFRTRIM_EN_A::_0)
    }
    #[doc = "Trim fine restarts from the IFR trim value whenever bus_reset/bus_resume is detected or module enable is desasserted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESTART_IFRTRIM_EN_A::_1)
    }
}
#[doc = "Field `RESET_RESUME_ROUGH_EN` reader - Reset/resume to rough phase enable"]
pub type RESET_RESUME_ROUGH_EN_R = crate::BitReader<RESET_RESUME_ROUGH_EN_A>;
#[doc = "Reset/resume to rough phase enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_RESUME_ROUGH_EN_A {
    #[doc = "0: Always works in tracking phase after the 1st time rough to track transition (default)"]
    _0 = 0,
    #[doc = "1: Go back to rough stage whenever bus reset or bus resume occurs"]
    _1 = 1,
}
impl From<RESET_RESUME_ROUGH_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_RESUME_ROUGH_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RESET_RESUME_ROUGH_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_RESUME_ROUGH_EN_A {
        match self.bits {
            false => RESET_RESUME_ROUGH_EN_A::_0,
            true => RESET_RESUME_ROUGH_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESET_RESUME_ROUGH_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESET_RESUME_ROUGH_EN_A::_1
    }
}
#[doc = "Field `RESET_RESUME_ROUGH_EN` writer - Reset/resume to rough phase enable"]
pub type RESET_RESUME_ROUGH_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CLK_RECOVER_CTRL_SPEC, RESET_RESUME_ROUGH_EN_A, O>;
impl<'a, const O: u8> RESET_RESUME_ROUGH_EN_W<'a, O> {
    #[doc = "Always works in tracking phase after the 1st time rough to track transition (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESET_RESUME_ROUGH_EN_A::_0)
    }
    #[doc = "Go back to rough stage whenever bus reset or bus resume occurs"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESET_RESUME_ROUGH_EN_A::_1)
    }
}
#[doc = "Field `CLOCK_RECOVER_EN` reader - Crystal-less USB enable"]
pub type CLOCK_RECOVER_EN_R = crate::BitReader<CLOCK_RECOVER_EN_A>;
#[doc = "Crystal-less USB enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLOCK_RECOVER_EN_A {
    #[doc = "0: Disable clock recovery block (default)"]
    _0 = 0,
    #[doc = "1: Enable clock recovery block"]
    _1 = 1,
}
impl From<CLOCK_RECOVER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLOCK_RECOVER_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLOCK_RECOVER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLOCK_RECOVER_EN_A {
        match self.bits {
            false => CLOCK_RECOVER_EN_A::_0,
            true => CLOCK_RECOVER_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLOCK_RECOVER_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLOCK_RECOVER_EN_A::_1
    }
}
#[doc = "Field `CLOCK_RECOVER_EN` writer - Crystal-less USB enable"]
pub type CLOCK_RECOVER_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CLK_RECOVER_CTRL_SPEC, CLOCK_RECOVER_EN_A, O>;
impl<'a, const O: u8> CLOCK_RECOVER_EN_W<'a, O> {
    #[doc = "Disable clock recovery block (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLOCK_RECOVER_EN_A::_0)
    }
    #[doc = "Enable clock recovery block"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLOCK_RECOVER_EN_A::_1)
    }
}
impl R {
    #[doc = "Bit 5 - Restart from IFR trim value"]
    #[inline(always)]
    pub fn restart_ifrtrim_en(&self) -> RESTART_IFRTRIM_EN_R {
        RESTART_IFRTRIM_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset/resume to rough phase enable"]
    #[inline(always)]
    pub fn reset_resume_rough_en(&self) -> RESET_RESUME_ROUGH_EN_R {
        RESET_RESUME_ROUGH_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Crystal-less USB enable"]
    #[inline(always)]
    pub fn clock_recover_en(&self) -> CLOCK_RECOVER_EN_R {
        CLOCK_RECOVER_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Restart from IFR trim value"]
    #[inline(always)]
    #[must_use]
    pub fn restart_ifrtrim_en(&mut self) -> RESTART_IFRTRIM_EN_W<5> {
        RESTART_IFRTRIM_EN_W::new(self)
    }
    #[doc = "Bit 6 - Reset/resume to rough phase enable"]
    #[inline(always)]
    #[must_use]
    pub fn reset_resume_rough_en(&mut self) -> RESET_RESUME_ROUGH_EN_W<6> {
        RESET_RESUME_ROUGH_EN_W::new(self)
    }
    #[doc = "Bit 7 - Crystal-less USB enable"]
    #[inline(always)]
    #[must_use]
    pub fn clock_recover_en(&mut self) -> CLOCK_RECOVER_EN_W<7> {
        CLOCK_RECOVER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Clock recovery control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_recover_ctrl](index.html) module"]
pub struct CLK_RECOVER_CTRL_SPEC;
impl crate::RegisterSpec for CLK_RECOVER_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clk_recover_ctrl::R](R) reader structure"]
impl crate::Readable for CLK_RECOVER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_recover_ctrl::W](W) writer structure"]
impl crate::Writable for CLK_RECOVER_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_RECOVER_CTRL to value 0"]
impl crate::Resettable for CLK_RECOVER_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
