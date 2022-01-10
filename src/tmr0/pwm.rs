#[doc = "Register `PWM` reader"]
pub struct R(crate::R<PWM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM` writer"]
pub struct W(crate::W<PWM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_SPEC>;
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
impl From<crate::W<PWM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_SPEC>) -> Self {
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
#[doc = "PWM. This register stores the value that is compared to the current timer count.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm](index.html) module"]
pub struct PWM_SPEC;
impl crate::RegisterSpec for PWM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm::R](R) reader structure"]
impl crate::Readable for PWM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm::W](W) writer structure"]
impl crate::Writable for PWM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM to value 0"]
impl crate::Resettable for PWM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
