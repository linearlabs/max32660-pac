#[doc = "Register `MPRI0` reader"]
pub struct R(crate::R<MPRI0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPRI0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPRI0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPRI0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPRI0` writer"]
pub struct W(crate::W<MPRI0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPRI0_SPEC>;
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
impl From<crate::W<MPRI0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPRI0_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Priority Control Register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpri0](index.html) module"]
pub struct MPRI0_SPEC;
impl crate::RegisterSpec for MPRI0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpri0::R](R) reader structure"]
impl crate::Readable for MPRI0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpri0::W](W) writer structure"]
impl crate::Writable for MPRI0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPRI0 to value 0x1414"]
impl crate::Resettable for MPRI0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1414
    }
}
