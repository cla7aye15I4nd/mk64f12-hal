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
#[doc = "Field `EDBG` reader - Enable Debug"]
pub type EDBG_R = crate::BitReader<EDBG_A>;
#[doc = "Enable Debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDBG_A {
    #[doc = "0: When in debug mode, the DMA continues to operate."]
    _0 = 0,
    #[doc = "1: When in debug mode, the DMA stalls the start of a new channel. Executing channels are allowed to complete. Channel execution resumes when the system exits debug mode or the EDBG bit is cleared."]
    _1 = 1,
}
impl From<EDBG_A> for bool {
    #[inline(always)]
    fn from(variant: EDBG_A) -> Self {
        variant as u8 != 0
    }
}
impl EDBG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDBG_A {
        match self.bits {
            false => EDBG_A::_0,
            true => EDBG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDBG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDBG_A::_1
    }
}
#[doc = "Field `EDBG` writer - Enable Debug"]
pub type EDBG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EDBG_A, O>;
impl<'a, const O: u8> EDBG_W<'a, O> {
    #[doc = "When in debug mode, the DMA continues to operate."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDBG_A::_0)
    }
    #[doc = "When in debug mode, the DMA stalls the start of a new channel. Executing channels are allowed to complete. Channel execution resumes when the system exits debug mode or the EDBG bit is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDBG_A::_1)
    }
}
#[doc = "Field `ERCA` reader - Enable Round Robin Channel Arbitration"]
pub type ERCA_R = crate::BitReader<ERCA_A>;
#[doc = "Enable Round Robin Channel Arbitration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERCA_A {
    #[doc = "0: Fixed priority arbitration is used for channel selection ."]
    _0 = 0,
    #[doc = "1: Round robin arbitration is used for channel selection ."]
    _1 = 1,
}
impl From<ERCA_A> for bool {
    #[inline(always)]
    fn from(variant: ERCA_A) -> Self {
        variant as u8 != 0
    }
}
impl ERCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERCA_A {
        match self.bits {
            false => ERCA_A::_0,
            true => ERCA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERCA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERCA_A::_1
    }
}
#[doc = "Field `ERCA` writer - Enable Round Robin Channel Arbitration"]
pub type ERCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ERCA_A, O>;
impl<'a, const O: u8> ERCA_W<'a, O> {
    #[doc = "Fixed priority arbitration is used for channel selection ."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERCA_A::_0)
    }
    #[doc = "Round robin arbitration is used for channel selection ."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERCA_A::_1)
    }
}
#[doc = "Field `HOE` reader - Halt On Error"]
pub type HOE_R = crate::BitReader<HOE_A>;
#[doc = "Halt On Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOE_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared."]
    _1 = 1,
}
impl From<HOE_A> for bool {
    #[inline(always)]
    fn from(variant: HOE_A) -> Self {
        variant as u8 != 0
    }
}
impl HOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOE_A {
        match self.bits {
            false => HOE_A::_0,
            true => HOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HOE_A::_1
    }
}
#[doc = "Field `HOE` writer - Halt On Error"]
pub type HOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HOE_A, O>;
impl<'a, const O: u8> HOE_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HOE_A::_0)
    }
    #[doc = "Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HOE_A::_1)
    }
}
#[doc = "Field `HALT` reader - Halt DMA Operations"]
pub type HALT_R = crate::BitReader<HALT_A>;
#[doc = "Halt DMA Operations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALT_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared."]
    _1 = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
impl HALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::_0,
            true => HALT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HALT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HALT_A::_1
    }
}
#[doc = "Field `HALT` writer - Halt DMA Operations"]
pub type HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HALT_A, O>;
impl<'a, const O: u8> HALT_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HALT_A::_0)
    }
    #[doc = "Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HALT_A::_1)
    }
}
#[doc = "Field `CLM` reader - Continuous Link Mode"]
pub type CLM_R = crate::BitReader<CLM_A>;
#[doc = "Continuous Link Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLM_A {
    #[doc = "0: A minor loop channel link made to itself goes through channel arbitration before being activated again."]
    _0 = 0,
    #[doc = "1: A minor loop channel link made to itself does not go through channel arbitration before being activated again. Upon minor loop completion, the channel activates again if that channel has a minor loop channel link enabled and the link channel is itself. This effectively applies the minor loop offsets and restarts the next minor loop."]
    _1 = 1,
}
impl From<CLM_A> for bool {
    #[inline(always)]
    fn from(variant: CLM_A) -> Self {
        variant as u8 != 0
    }
}
impl CLM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLM_A {
        match self.bits {
            false => CLM_A::_0,
            true => CLM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLM_A::_1
    }
}
#[doc = "Field `CLM` writer - Continuous Link Mode"]
pub type CLM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CLM_A, O>;
impl<'a, const O: u8> CLM_W<'a, O> {
    #[doc = "A minor loop channel link made to itself goes through channel arbitration before being activated again."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLM_A::_0)
    }
    #[doc = "A minor loop channel link made to itself does not go through channel arbitration before being activated again. Upon minor loop completion, the channel activates again if that channel has a minor loop channel link enabled and the link channel is itself. This effectively applies the minor loop offsets and restarts the next minor loop."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLM_A::_1)
    }
}
#[doc = "Field `EMLM` reader - Enable Minor Loop Mapping"]
pub type EMLM_R = crate::BitReader<EMLM_A>;
#[doc = "Enable Minor Loop Mapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMLM_A {
    #[doc = "0: Disabled. TCDn.word2 is defined as a 32-bit NBYTES field."]
    _0 = 0,
    #[doc = "1: Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field. The individual enable fields allow the minor loop offset to be applied to the source address, the destination address, or both. The NBYTES field is reduced when either offset is enabled."]
    _1 = 1,
}
impl From<EMLM_A> for bool {
    #[inline(always)]
    fn from(variant: EMLM_A) -> Self {
        variant as u8 != 0
    }
}
impl EMLM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMLM_A {
        match self.bits {
            false => EMLM_A::_0,
            true => EMLM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EMLM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EMLM_A::_1
    }
}
#[doc = "Field `EMLM` writer - Enable Minor Loop Mapping"]
pub type EMLM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EMLM_A, O>;
impl<'a, const O: u8> EMLM_W<'a, O> {
    #[doc = "Disabled. TCDn.word2 is defined as a 32-bit NBYTES field."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EMLM_A::_0)
    }
    #[doc = "Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field. The individual enable fields allow the minor loop offset to be applied to the source address, the destination address, or both. The NBYTES field is reduced when either offset is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EMLM_A::_1)
    }
}
#[doc = "Field `ECX` reader - Error Cancel Transfer"]
pub type ECX_R = crate::BitReader<ECX_A>;
#[doc = "Error Cancel Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECX_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Cancel the remaining data transfer in the same fashion as the CX bit. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The ECX bit clears itself after the cancel is honored. In addition to cancelling the transfer, ECX treats the cancel as an error condition, thus updating the Error Status register (DMAx_ES) and generating an optional error interrupt."]
    _1 = 1,
}
impl From<ECX_A> for bool {
    #[inline(always)]
    fn from(variant: ECX_A) -> Self {
        variant as u8 != 0
    }
}
impl ECX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECX_A {
        match self.bits {
            false => ECX_A::_0,
            true => ECX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECX_A::_1
    }
}
#[doc = "Field `ECX` writer - Error Cancel Transfer"]
pub type ECX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ECX_A, O>;
impl<'a, const O: u8> ECX_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECX_A::_0)
    }
    #[doc = "Cancel the remaining data transfer in the same fashion as the CX bit. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The ECX bit clears itself after the cancel is honored. In addition to cancelling the transfer, ECX treats the cancel as an error condition, thus updating the Error Status register (DMAx_ES) and generating an optional error interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECX_A::_1)
    }
}
#[doc = "Field `CX` reader - Cancel Transfer"]
pub type CX_R = crate::BitReader<CX_A>;
#[doc = "Cancel Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CX_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Cancel the remaining data transfer. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The CX bit clears itself after the cancel has been honored. This cancel retires the channel normally as if the minor loop was completed."]
    _1 = 1,
}
impl From<CX_A> for bool {
    #[inline(always)]
    fn from(variant: CX_A) -> Self {
        variant as u8 != 0
    }
}
impl CX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CX_A {
        match self.bits {
            false => CX_A::_0,
            true => CX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CX_A::_1
    }
}
#[doc = "Field `CX` writer - Cancel Transfer"]
pub type CX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CX_A, O>;
impl<'a, const O: u8> CX_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CX_A::_0)
    }
    #[doc = "Cancel the remaining data transfer. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The CX bit clears itself after the cancel has been honored. This cancel retires the channel normally as if the minor loop was completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CX_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - Enable Debug"]
    #[inline(always)]
    pub fn edbg(&self) -> EDBG_R {
        EDBG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Round Robin Channel Arbitration"]
    #[inline(always)]
    pub fn erca(&self) -> ERCA_R {
        ERCA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Halt On Error"]
    #[inline(always)]
    pub fn hoe(&self) -> HOE_R {
        HOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Halt DMA Operations"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Continuous Link Mode"]
    #[inline(always)]
    pub fn clm(&self) -> CLM_R {
        CLM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Minor Loop Mapping"]
    #[inline(always)]
    pub fn emlm(&self) -> EMLM_R {
        EMLM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Error Cancel Transfer"]
    #[inline(always)]
    pub fn ecx(&self) -> ECX_R {
        ECX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Cancel Transfer"]
    #[inline(always)]
    pub fn cx(&self) -> CX_R {
        CX_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable Debug"]
    #[inline(always)]
    #[must_use]
    pub fn edbg(&mut self) -> EDBG_W<1> {
        EDBG_W::new(self)
    }
    #[doc = "Bit 2 - Enable Round Robin Channel Arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn erca(&mut self) -> ERCA_W<2> {
        ERCA_W::new(self)
    }
    #[doc = "Bit 4 - Halt On Error"]
    #[inline(always)]
    #[must_use]
    pub fn hoe(&mut self) -> HOE_W<4> {
        HOE_W::new(self)
    }
    #[doc = "Bit 5 - Halt DMA Operations"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<5> {
        HALT_W::new(self)
    }
    #[doc = "Bit 6 - Continuous Link Mode"]
    #[inline(always)]
    #[must_use]
    pub fn clm(&mut self) -> CLM_W<6> {
        CLM_W::new(self)
    }
    #[doc = "Bit 7 - Enable Minor Loop Mapping"]
    #[inline(always)]
    #[must_use]
    pub fn emlm(&mut self) -> EMLM_W<7> {
        EMLM_W::new(self)
    }
    #[doc = "Bit 16 - Error Cancel Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ecx(&mut self) -> ECX_W<16> {
        ECX_W::new(self)
    }
    #[doc = "Bit 17 - Cancel Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn cx(&mut self) -> CX_W<17> {
        CX_W::new(self)
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
