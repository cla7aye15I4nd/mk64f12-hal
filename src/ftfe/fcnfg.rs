#[doc = "Register `FCNFG` reader"]
pub struct R(crate::R<FCNFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCNFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCNFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCNFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCNFG` writer"]
pub struct W(crate::W<FCNFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCNFG_SPEC>;
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
impl From<crate::W<FCNFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCNFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEERDY` reader - For devices with FlexNVM: This flag indicates if the EEPROM backup data has been copied to the FlexRAM and is therefore available for read access"]
pub type EEERDY_R = crate::BitReader<EEERDY_A>;
#[doc = "For devices with FlexNVM: This flag indicates if the EEPROM backup data has been copied to the FlexRAM and is therefore available for read access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEERDY_A {
    #[doc = "0: For devices with FlexNVM: FlexRAM is not available for EEPROM operation."]
    _0 = 0,
    #[doc = "1: For devices with FlexNVM: FlexRAM is available for EEPROM operations where: reads from the FlexRAM return data previously written to the FlexRAM in EEPROM mode and writes launch an EEPROM operation to store the written data in the FlexRAM and EEPROM backup."]
    _1 = 1,
}
impl From<EEERDY_A> for bool {
    #[inline(always)]
    fn from(variant: EEERDY_A) -> Self {
        variant as u8 != 0
    }
}
impl EEERDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEERDY_A {
        match self.bits {
            false => EEERDY_A::_0,
            true => EEERDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEERDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEERDY_A::_1
    }
}
#[doc = "Field `RAMRDY` reader - RAM Ready"]
pub type RAMRDY_R = crate::BitReader<RAMRDY_A>;
#[doc = "RAM Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMRDY_A {
    #[doc = "0: For devices with FlexNVM: FlexRAM is not available for traditional RAM access. For devices without FlexNVM: Programming acceleration RAM is not available."]
    _0 = 0,
    #[doc = "1: For devices with FlexNVM: FlexRAM is available as traditional RAM only; writes to the FlexRAM do not trigger EEPROM operations. For devices without FlexNVM: Programming acceleration RAM is available."]
    _1 = 1,
}
impl From<RAMRDY_A> for bool {
    #[inline(always)]
    fn from(variant: RAMRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMRDY_A {
        match self.bits {
            false => RAMRDY_A::_0,
            true => RAMRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RAMRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RAMRDY_A::_1
    }
}
#[doc = "Field `PFLSH` reader - FTFE configuration"]
pub type PFLSH_R = crate::BitReader<PFLSH_A>;
#[doc = "FTFE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFLSH_A {
    #[doc = "0: For devices with FlexNVM: FTFE configuration supports two program flash blocks and two FlexNVM blocks For devices with program flash only: Reserved"]
    _0 = 0,
    #[doc = "1: For devices with FlexNVM: Reserved For devices with program flash only: FTFE configuration supports four program flash blocks"]
    _1 = 1,
}
impl From<PFLSH_A> for bool {
    #[inline(always)]
    fn from(variant: PFLSH_A) -> Self {
        variant as u8 != 0
    }
}
impl PFLSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFLSH_A {
        match self.bits {
            false => PFLSH_A::_0,
            true => PFLSH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFLSH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFLSH_A::_1
    }
}
#[doc = "Field `SWAP` reader - Swap"]
pub type SWAP_R = crate::BitReader<SWAP_A>;
#[doc = "Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAP_A {
    #[doc = "0: For devices with FlexNVM: Program flash 0 block is located at relative address 0x0000 For devices with program flash only: Program flash 0 block is located at relative address 0x0000"]
    _0 = 0,
    #[doc = "1: For devices with FlexNVM: Reserved For devices with program flash only: Program flash 1 block is located at relative address 0x0000"]
    _1 = 1,
}
impl From<SWAP_A> for bool {
    #[inline(always)]
    fn from(variant: SWAP_A) -> Self {
        variant as u8 != 0
    }
}
impl SWAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWAP_A {
        match self.bits {
            false => SWAP_A::_0,
            true => SWAP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWAP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWAP_A::_1
    }
}
#[doc = "Field `ERSSUSP` reader - Erase Suspend"]
pub type ERSSUSP_R = crate::BitReader<ERSSUSP_A>;
#[doc = "Erase Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERSSUSP_A {
    #[doc = "0: No suspend requested"]
    _0 = 0,
    #[doc = "1: Suspend the current Erase Flash Sector command execution."]
    _1 = 1,
}
impl From<ERSSUSP_A> for bool {
    #[inline(always)]
    fn from(variant: ERSSUSP_A) -> Self {
        variant as u8 != 0
    }
}
impl ERSSUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERSSUSP_A {
        match self.bits {
            false => ERSSUSP_A::_0,
            true => ERSSUSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERSSUSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERSSUSP_A::_1
    }
}
#[doc = "Field `ERSSUSP` writer - Erase Suspend"]
pub type ERSSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u8, FCNFG_SPEC, ERSSUSP_A, O>;
impl<'a, const O: u8> ERSSUSP_W<'a, O> {
    #[doc = "No suspend requested"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERSSUSP_A::_0)
    }
    #[doc = "Suspend the current Erase Flash Sector command execution."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERSSUSP_A::_1)
    }
}
#[doc = "Field `ERSAREQ` reader - Erase All Request"]
pub type ERSAREQ_R = crate::BitReader<ERSAREQ_A>;
#[doc = "Erase All Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERSAREQ_A {
    #[doc = "0: No request or request complete"]
    _0 = 0,
    #[doc = "1: Request to: run the Erase All Blocks command, verify the erased state, program the security byte in the Flash Configuration Field to the unsecure state, and release MCU security by setting the FSEC\\[SEC\\]
field to the unsecure state."]
    _1 = 1,
}
impl From<ERSAREQ_A> for bool {
    #[inline(always)]
    fn from(variant: ERSAREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl ERSAREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERSAREQ_A {
        match self.bits {
            false => ERSAREQ_A::_0,
            true => ERSAREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERSAREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERSAREQ_A::_1
    }
}
#[doc = "Field `RDCOLLIE` reader - Read Collision Error Interrupt Enable"]
pub type RDCOLLIE_R = crate::BitReader<RDCOLLIE_A>;
#[doc = "Read Collision Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDCOLLIE_A {
    #[doc = "0: Read collision error interrupt disabled"]
    _0 = 0,
    #[doc = "1: Read collision error interrupt enabled. An interrupt request is generated whenever an FTFE read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
    _1 = 1,
}
impl From<RDCOLLIE_A> for bool {
    #[inline(always)]
    fn from(variant: RDCOLLIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RDCOLLIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDCOLLIE_A {
        match self.bits {
            false => RDCOLLIE_A::_0,
            true => RDCOLLIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDCOLLIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDCOLLIE_A::_1
    }
}
#[doc = "Field `RDCOLLIE` writer - Read Collision Error Interrupt Enable"]
pub type RDCOLLIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, FCNFG_SPEC, RDCOLLIE_A, O>;
impl<'a, const O: u8> RDCOLLIE_W<'a, O> {
    #[doc = "Read collision error interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDCOLLIE_A::_0)
    }
    #[doc = "Read collision error interrupt enabled. An interrupt request is generated whenever an FTFE read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDCOLLIE_A::_1)
    }
}
#[doc = "Field `CCIE` reader - Command Complete Interrupt Enable"]
pub type CCIE_R = crate::BitReader<CCIE_A>;
#[doc = "Command Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCIE_A {
    #[doc = "0: Command complete interrupt disabled"]
    _0 = 0,
    #[doc = "1: Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\]
flag is set."]
    _1 = 1,
}
impl From<CCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIE_A {
        match self.bits {
            false => CCIE_A::_0,
            true => CCIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCIE_A::_1
    }
}
#[doc = "Field `CCIE` writer - Command Complete Interrupt Enable"]
pub type CCIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, FCNFG_SPEC, CCIE_A, O>;
impl<'a, const O: u8> CCIE_W<'a, O> {
    #[doc = "Command complete interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCIE_A::_0)
    }
    #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\]
flag is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - For devices with FlexNVM: This flag indicates if the EEPROM backup data has been copied to the FlexRAM and is therefore available for read access"]
    #[inline(always)]
    pub fn eeerdy(&self) -> EEERDY_R {
        EEERDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM Ready"]
    #[inline(always)]
    pub fn ramrdy(&self) -> RAMRDY_R {
        RAMRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FTFE configuration"]
    #[inline(always)]
    pub fn pflsh(&self) -> PFLSH_R {
        PFLSH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Swap"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Erase Suspend"]
    #[inline(always)]
    pub fn erssusp(&self) -> ERSSUSP_R {
        ERSSUSP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Erase All Request"]
    #[inline(always)]
    pub fn ersareq(&self) -> ERSAREQ_R {
        ERSAREQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
    #[inline(always)]
    pub fn rdcollie(&self) -> RDCOLLIE_R {
        RDCOLLIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Erase Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn erssusp(&mut self) -> ERSSUSP_W<4> {
        ERSSUSP_W::new(self)
    }
    #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdcollie(&mut self) -> RDCOLLIE_W<6> {
        RDCOLLIE_W::new(self)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccie(&mut self) -> CCIE_W<7> {
        CCIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcnfg](index.html) module"]
pub struct FCNFG_SPEC;
impl crate::RegisterSpec for FCNFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fcnfg::R](R) reader structure"]
impl crate::Readable for FCNFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcnfg::W](W) writer structure"]
impl crate::Writable for FCNFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCNFG to value 0"]
impl crate::Resettable for FCNFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
