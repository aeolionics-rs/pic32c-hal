#[doc = "Register `DISABLE` reader"]
pub type R = crate::R<DisableSpec>;
#[doc = "Register `DISABLE` writer"]
pub type W = crate::W<DisableSpec>;
#[doc = "ENROLL Command Disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enrollselect {
    #[doc = "0: Command is enabled."]
    Enroll0 = 0,
    #[doc = "1: Command is disabled."]
    Enroll1 = 1,
}
impl From<Enrollselect> for bool {
    #[inline(always)]
    fn from(variant: Enrollselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENROLL` reader - ENROLL Command Disable bit"]
pub type EnrollR = crate::BitReader<Enrollselect>;
impl EnrollR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enrollselect {
        match self.bits {
            false => Enrollselect::Enroll0,
            true => Enrollselect::Enroll1,
        }
    }
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn is_enroll_0(&self) -> bool {
        *self == Enrollselect::Enroll0
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn is_enroll_1(&self) -> bool {
        *self == Enrollselect::Enroll1
    }
}
#[doc = "Field `ENROLL` writer - ENROLL Command Disable bit"]
pub type EnrollW<'a, REG> = crate::BitWriter1S<'a, REG, Enrollselect>;
impl<'a, REG> EnrollW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn enroll_0(self) -> &'a mut crate::W<REG> {
        self.variant(Enrollselect::Enroll0)
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn enroll_1(self) -> &'a mut crate::W<REG> {
        self.variant(Enrollselect::Enroll1)
    }
}
#[doc = "START Command Disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Startselect {
    #[doc = "0: Command is enabled."]
    Start0 = 0,
    #[doc = "1: Command is disabled."]
    Start1 = 1,
}
impl From<Startselect> for bool {
    #[inline(always)]
    fn from(variant: Startselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - START Command Disable bit"]
pub type StartR = crate::BitReader<Startselect>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Startselect {
        match self.bits {
            false => Startselect::Start0,
            true => Startselect::Start1,
        }
    }
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn is_start_0(&self) -> bool {
        *self == Startselect::Start0
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn is_start_1(&self) -> bool {
        *self == Startselect::Start1
    }
}
#[doc = "Field `START` writer - START Command Disable bit"]
pub type StartW<'a, REG> = crate::BitWriter1S<'a, REG, Startselect>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn start_0(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Start0)
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn start_1(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Start1)
    }
}
#[doc = "RECONSTRUCT Command Disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reconstructselect {
    #[doc = "0: Command is enabled."]
    Reconstruct0 = 0,
    #[doc = "1: Command is disabled."]
    Reconstruct1 = 1,
}
impl From<Reconstructselect> for bool {
    #[inline(always)]
    fn from(variant: Reconstructselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECONSTRUCT` reader - RECONSTRUCT Command Disable bit"]
pub type ReconstructR = crate::BitReader<Reconstructselect>;
impl ReconstructR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reconstructselect {
        match self.bits {
            false => Reconstructselect::Reconstruct0,
            true => Reconstructselect::Reconstruct1,
        }
    }
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn is_reconstruct_0(&self) -> bool {
        *self == Reconstructselect::Reconstruct0
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn is_reconstruct_1(&self) -> bool {
        *self == Reconstructselect::Reconstruct1
    }
}
#[doc = "Field `RECONSTRUCT` writer - RECONSTRUCT Command Disable bit"]
pub type ReconstructW<'a, REG> = crate::BitWriter1S<'a, REG, Reconstructselect>;
impl<'a, REG> ReconstructW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn reconstruct_0(self) -> &'a mut crate::W<REG> {
        self.variant(Reconstructselect::Reconstruct0)
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn reconstruct_1(self) -> &'a mut crate::W<REG> {
        self.variant(Reconstructselect::Reconstruct1)
    }
}
#[doc = "STOP Command Disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopselect {
    #[doc = "0: Command is enabled."]
    Stop0 = 0,
    #[doc = "1: Command is disabled."]
    Stop1 = 1,
}
impl From<Stopselect> for bool {
    #[inline(always)]
    fn from(variant: Stopselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - STOP Command Disable bit"]
pub type StopR = crate::BitReader<Stopselect>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopselect {
        match self.bits {
            false => Stopselect::Stop0,
            true => Stopselect::Stop1,
        }
    }
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn is_stop_0(&self) -> bool {
        *self == Stopselect::Stop0
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn is_stop_1(&self) -> bool {
        *self == Stopselect::Stop1
    }
}
#[doc = "Field `STOP` writer - STOP Command Disable bit"]
pub type StopW<'a, REG> = crate::BitWriter1S<'a, REG, Stopselect>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn stop_0(self) -> &'a mut crate::W<REG> {
        self.variant(Stopselect::Stop0)
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn stop_1(self) -> &'a mut crate::W<REG> {
        self.variant(Stopselect::Stop1)
    }
}
#[doc = "GET_KEY Command Disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GetKeyselect {
    #[doc = "0: Command is enabled."]
    GetKey0 = 0,
    #[doc = "1: Command is disabled."]
    GetKey1 = 1,
}
impl From<GetKeyselect> for bool {
    #[inline(always)]
    fn from(variant: GetKeyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GET_KEY` reader - GET_KEY Command Disable bit"]
pub type GetKeyR = crate::BitReader<GetKeyselect>;
impl GetKeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GetKeyselect {
        match self.bits {
            false => GetKeyselect::GetKey0,
            true => GetKeyselect::GetKey1,
        }
    }
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn is_get_key_0(&self) -> bool {
        *self == GetKeyselect::GetKey0
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn is_get_key_1(&self) -> bool {
        *self == GetKeyselect::GetKey1
    }
}
#[doc = "Field `GET_KEY` writer - GET_KEY Command Disable bit"]
pub type GetKeyW<'a, REG> = crate::BitWriter1S<'a, REG, GetKeyselect>;
impl<'a, REG> GetKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn get_key_0(self) -> &'a mut crate::W<REG> {
        self.variant(GetKeyselect::GetKey0)
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn get_key_1(self) -> &'a mut crate::W<REG> {
        self.variant(GetKeyselect::GetKey1)
    }
}
#[doc = "UNWRAP Command Disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unwrapselect {
    #[doc = "0: Command is enabled."]
    Unwrap0 = 0,
    #[doc = "1: Command is disabled."]
    Unwrap1 = 1,
}
impl From<Unwrapselect> for bool {
    #[inline(always)]
    fn from(variant: Unwrapselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNWRAP` reader - UNWRAP Command Disable bit"]
pub type UnwrapR = crate::BitReader<Unwrapselect>;
impl UnwrapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Unwrapselect {
        match self.bits {
            false => Unwrapselect::Unwrap0,
            true => Unwrapselect::Unwrap1,
        }
    }
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn is_unwrap_0(&self) -> bool {
        *self == Unwrapselect::Unwrap0
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn is_unwrap_1(&self) -> bool {
        *self == Unwrapselect::Unwrap1
    }
}
#[doc = "Field `UNWRAP` writer - UNWRAP Command Disable bit"]
pub type UnwrapW<'a, REG> = crate::BitWriter1S<'a, REG, Unwrapselect>;
impl<'a, REG> UnwrapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn unwrap_0(self) -> &'a mut crate::W<REG> {
        self.variant(Unwrapselect::Unwrap0)
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn unwrap_1(self) -> &'a mut crate::W<REG> {
        self.variant(Unwrapselect::Unwrap1)
    }
}
#[doc = "WRAP_GENERATE_RANDOM Command Disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrapGenRandselect {
    #[doc = "0: Command is enabled."]
    WrapGenRand0 = 0,
    #[doc = "1: Command is disabled."]
    WrapGenRand1 = 1,
}
impl From<WrapGenRandselect> for bool {
    #[inline(always)]
    fn from(variant: WrapGenRandselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRAP_GEN_RAND` reader - WRAP_GENERATE_RANDOM Command Disable bit"]
pub type WrapGenRandR = crate::BitReader<WrapGenRandselect>;
impl WrapGenRandR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WrapGenRandselect {
        match self.bits {
            false => WrapGenRandselect::WrapGenRand0,
            true => WrapGenRandselect::WrapGenRand1,
        }
    }
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn is_wrap_gen_rand_0(&self) -> bool {
        *self == WrapGenRandselect::WrapGenRand0
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn is_wrap_gen_rand_1(&self) -> bool {
        *self == WrapGenRandselect::WrapGenRand1
    }
}
#[doc = "Field `WRAP_GEN_RAND` writer - WRAP_GENERATE_RANDOM Command Disable bit"]
pub type WrapGenRandW<'a, REG> = crate::BitWriter1S<'a, REG, WrapGenRandselect>;
impl<'a, REG> WrapGenRandW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn wrap_gen_rand_0(self) -> &'a mut crate::W<REG> {
        self.variant(WrapGenRandselect::WrapGenRand0)
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn wrap_gen_rand_1(self) -> &'a mut crate::W<REG> {
        self.variant(WrapGenRandselect::WrapGenRand1)
    }
}
#[doc = "WRAP Command Disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrapselect {
    #[doc = "0: Command is enabled."]
    Wrap0 = 0,
    #[doc = "1: Command is disabled."]
    Wrap1 = 1,
}
impl From<Wrapselect> for bool {
    #[inline(always)]
    fn from(variant: Wrapselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRAP` reader - WRAP Command Disable bit"]
pub type WrapR = crate::BitReader<Wrapselect>;
impl WrapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrapselect {
        match self.bits {
            false => Wrapselect::Wrap0,
            true => Wrapselect::Wrap1,
        }
    }
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn is_wrap_0(&self) -> bool {
        *self == Wrapselect::Wrap0
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn is_wrap_1(&self) -> bool {
        *self == Wrapselect::Wrap1
    }
}
#[doc = "Field `WRAP` writer - WRAP Command Disable bit"]
pub type WrapW<'a, REG> = crate::BitWriter1S<'a, REG, Wrapselect>;
impl<'a, REG> WrapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn wrap_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wrapselect::Wrap0)
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn wrap_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wrapselect::Wrap1)
    }
}
#[doc = "GENERATE_RANDOM Command Disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenRandselect {
    #[doc = "0: Command is enabled."]
    GenRand0 = 0,
    #[doc = "1: Command is disabled."]
    GenRand1 = 1,
}
impl From<GenRandselect> for bool {
    #[inline(always)]
    fn from(variant: GenRandselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GEN_RAND` reader - GENERATE_RANDOM Command Disable bit"]
pub type GenRandR = crate::BitReader<GenRandselect>;
impl GenRandR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GenRandselect {
        match self.bits {
            false => GenRandselect::GenRand0,
            true => GenRandselect::GenRand1,
        }
    }
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn is_gen_rand_0(&self) -> bool {
        *self == GenRandselect::GenRand0
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn is_gen_rand_1(&self) -> bool {
        *self == GenRandselect::GenRand1
    }
}
#[doc = "Field `GEN_RAND` writer - GENERATE_RANDOM Command Disable bit"]
pub type GenRandW<'a, REG> = crate::BitWriter1S<'a, REG, GenRandselect>;
impl<'a, REG> GenRandW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn gen_rand_0(self) -> &'a mut crate::W<REG> {
        self.variant(GenRandselect::GenRand0)
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn gen_rand_1(self) -> &'a mut crate::W<REG> {
        self.variant(GenRandselect::GenRand1)
    }
}
#[doc = "SKIP_MEMORY TEST Command Control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkipMemTestselect {
    #[doc = "0: Command is enabled / Not Skipped."]
    SkipMemTest0 = 0,
    #[doc = "1: Command is disabled / Skipped during initialization"]
    SkipMemTest1 = 1,
}
impl From<SkipMemTestselect> for bool {
    #[inline(always)]
    fn from(variant: SkipMemTestselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SKIP_MEM_TEST` reader - SKIP_MEMORY TEST Command Control bit"]
pub type SkipMemTestR = crate::BitReader<SkipMemTestselect>;
impl SkipMemTestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SkipMemTestselect {
        match self.bits {
            false => SkipMemTestselect::SkipMemTest0,
            true => SkipMemTestselect::SkipMemTest1,
        }
    }
    #[doc = "Command is enabled / Not Skipped."]
    #[inline(always)]
    pub fn is_skip_mem_test_0(&self) -> bool {
        *self == SkipMemTestselect::SkipMemTest0
    }
    #[doc = "Command is disabled / Skipped during initialization"]
    #[inline(always)]
    pub fn is_skip_mem_test_1(&self) -> bool {
        *self == SkipMemTestselect::SkipMemTest1
    }
}
#[doc = "Field `SKIP_MEM_TEST` writer - SKIP_MEMORY TEST Command Control bit"]
pub type SkipMemTestW<'a, REG> = crate::BitWriter1S<'a, REG, SkipMemTestselect>;
impl<'a, REG> SkipMemTestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command is enabled / Not Skipped."]
    #[inline(always)]
    pub fn skip_mem_test_0(self) -> &'a mut crate::W<REG> {
        self.variant(SkipMemTestselect::SkipMemTest0)
    }
    #[doc = "Command is disabled / Skipped during initialization"]
    #[inline(always)]
    pub fn skip_mem_test_1(self) -> &'a mut crate::W<REG> {
        self.variant(SkipMemTestselect::SkipMemTest1)
    }
}
#[doc = "TEST_MEMORY Command Disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestMemselect {
    #[doc = "0: Command is enabled."]
    TestMem0 = 0,
    #[doc = "1: Command is disabled."]
    TestMem1 = 1,
}
impl From<TestMemselect> for bool {
    #[inline(always)]
    fn from(variant: TestMemselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST_MEM` reader - TEST_MEMORY Command Disable bit"]
pub type TestMemR = crate::BitReader<TestMemselect>;
impl TestMemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TestMemselect {
        match self.bits {
            false => TestMemselect::TestMem0,
            true => TestMemselect::TestMem1,
        }
    }
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn is_test_mem_0(&self) -> bool {
        *self == TestMemselect::TestMem0
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn is_test_mem_1(&self) -> bool {
        *self == TestMemselect::TestMem1
    }
}
#[doc = "Field `TEST_MEM` writer - TEST_MEMORY Command Disable bit"]
pub type TestMemW<'a, REG> = crate::BitWriter1S<'a, REG, TestMemselect>;
impl<'a, REG> TestMemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn test_mem_0(self) -> &'a mut crate::W<REG> {
        self.variant(TestMemselect::TestMem0)
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn test_mem_1(self) -> &'a mut crate::W<REG> {
        self.variant(TestMemselect::TestMem1)
    }
}
#[doc = "TEST_PUF Command Disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestPufselect {
    #[doc = "0: Command is enabled."]
    TestPuf0 = 0,
    #[doc = "1: Command is disabled."]
    TestPuf1 = 1,
}
impl From<TestPufselect> for bool {
    #[inline(always)]
    fn from(variant: TestPufselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST_PUF` reader - TEST_PUF Command Disable bit"]
pub type TestPufR = crate::BitReader<TestPufselect>;
impl TestPufR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TestPufselect {
        match self.bits {
            false => TestPufselect::TestPuf0,
            true => TestPufselect::TestPuf1,
        }
    }
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn is_test_puf_0(&self) -> bool {
        *self == TestPufselect::TestPuf0
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn is_test_puf_1(&self) -> bool {
        *self == TestPufselect::TestPuf1
    }
}
#[doc = "Field `TEST_PUF` writer - TEST_PUF Command Disable bit"]
pub type TestPufW<'a, REG> = crate::BitWriter1S<'a, REG, TestPufselect>;
impl<'a, REG> TestPufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command is enabled."]
    #[inline(always)]
    pub fn test_puf_0(self) -> &'a mut crate::W<REG> {
        self.variant(TestPufselect::TestPuf0)
    }
    #[doc = "Command is disabled."]
    #[inline(always)]
    pub fn test_puf_1(self) -> &'a mut crate::W<REG> {
        self.variant(TestPufselect::TestPuf1)
    }
}
impl R {
    #[doc = "Bit 1 - ENROLL Command Disable bit"]
    #[inline(always)]
    pub fn enroll(&self) -> EnrollR {
        EnrollR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - START Command Disable bit"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RECONSTRUCT Command Disable bit"]
    #[inline(always)]
    pub fn reconstruct(&self) -> ReconstructR {
        ReconstructR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - STOP Command Disable bit"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GET_KEY Command Disable bit"]
    #[inline(always)]
    pub fn get_key(&self) -> GetKeyR {
        GetKeyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UNWRAP Command Disable bit"]
    #[inline(always)]
    pub fn unwrap(&self) -> UnwrapR {
        UnwrapR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WRAP_GENERATE_RANDOM Command Disable bit"]
    #[inline(always)]
    pub fn wrap_gen_rand(&self) -> WrapGenRandR {
        WrapGenRandR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WRAP Command Disable bit"]
    #[inline(always)]
    pub fn wrap(&self) -> WrapR {
        WrapR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - GENERATE_RANDOM Command Disable bit"]
    #[inline(always)]
    pub fn gen_rand(&self) -> GenRandR {
        GenRandR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 29 - SKIP_MEMORY TEST Command Control bit"]
    #[inline(always)]
    pub fn skip_mem_test(&self) -> SkipMemTestR {
        SkipMemTestR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TEST_MEMORY Command Disable bit"]
    #[inline(always)]
    pub fn test_mem(&self) -> TestMemR {
        TestMemR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TEST_PUF Command Disable bit"]
    #[inline(always)]
    pub fn test_puf(&self) -> TestPufR {
        TestPufR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ENROLL Command Disable bit"]
    #[inline(always)]
    pub fn enroll(&mut self) -> EnrollW<'_, DisableSpec> {
        EnrollW::new(self, 1)
    }
    #[doc = "Bit 2 - START Command Disable bit"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, DisableSpec> {
        StartW::new(self, 2)
    }
    #[doc = "Bit 3 - RECONSTRUCT Command Disable bit"]
    #[inline(always)]
    pub fn reconstruct(&mut self) -> ReconstructW<'_, DisableSpec> {
        ReconstructW::new(self, 3)
    }
    #[doc = "Bit 5 - STOP Command Disable bit"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, DisableSpec> {
        StopW::new(self, 5)
    }
    #[doc = "Bit 6 - GET_KEY Command Disable bit"]
    #[inline(always)]
    pub fn get_key(&mut self) -> GetKeyW<'_, DisableSpec> {
        GetKeyW::new(self, 6)
    }
    #[doc = "Bit 7 - UNWRAP Command Disable bit"]
    #[inline(always)]
    pub fn unwrap(&mut self) -> UnwrapW<'_, DisableSpec> {
        UnwrapW::new(self, 7)
    }
    #[doc = "Bit 8 - WRAP_GENERATE_RANDOM Command Disable bit"]
    #[inline(always)]
    pub fn wrap_gen_rand(&mut self) -> WrapGenRandW<'_, DisableSpec> {
        WrapGenRandW::new(self, 8)
    }
    #[doc = "Bit 9 - WRAP Command Disable bit"]
    #[inline(always)]
    pub fn wrap(&mut self) -> WrapW<'_, DisableSpec> {
        WrapW::new(self, 9)
    }
    #[doc = "Bit 15 - GENERATE_RANDOM Command Disable bit"]
    #[inline(always)]
    pub fn gen_rand(&mut self) -> GenRandW<'_, DisableSpec> {
        GenRandW::new(self, 15)
    }
    #[doc = "Bit 29 - SKIP_MEMORY TEST Command Control bit"]
    #[inline(always)]
    pub fn skip_mem_test(&mut self) -> SkipMemTestW<'_, DisableSpec> {
        SkipMemTestW::new(self, 29)
    }
    #[doc = "Bit 30 - TEST_MEMORY Command Disable bit"]
    #[inline(always)]
    pub fn test_mem(&mut self) -> TestMemW<'_, DisableSpec> {
        TestMemW::new(self, 30)
    }
    #[doc = "Bit 31 - TEST_PUF Command Disable bit"]
    #[inline(always)]
    pub fn test_puf(&mut self) -> TestPufW<'_, DisableSpec> {
        TestPufW::new(self, 31)
    }
}
#[doc = "Disable Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`disable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisableSpec;
impl crate::RegisterSpec for DisableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`disable::R`](R) reader structure"]
impl crate::Readable for DisableSpec {}
#[doc = "`write(|w| ..)` method takes [`disable::W`](W) writer structure"]
impl crate::Writable for DisableSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xe000_83ee;
}
#[doc = "`reset()` method sets DISABLE to value 0"]
impl crate::Resettable for DisableSpec {}
