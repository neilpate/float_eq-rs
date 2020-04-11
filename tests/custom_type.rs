use float_eq::{FloatDiff, FloatEq, FloatEqDebug};

//------------------------------------------------------------------------------
// MyComplex32
//------------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

//------------------------------------------------------------------------------
// FloatDiff with Self
//------------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
struct MyComplex32UlpsDiff {
    re: u32,
    im: u32,
}

impl FloatDiff for MyComplex32 {
    type AbsDiff = Self;
    type UlpsDiff = MyComplex32UlpsDiff;

    fn abs_diff(&self, other: &Self) -> Self::AbsDiff {
        MyComplex32 {
            re: self.re.abs_diff(&other.re),
            im: self.im.abs_diff(&other.im),
        }
    }

    fn ulps_diff(&self, other: &Self) -> Self::UlpsDiff {
        MyComplex32UlpsDiff {
            re: self.re.ulps_diff(&other.re),
            im: self.im.ulps_diff(&other.im),
        }
    }
}

#[test]
fn float_diff_self() {
    let a = MyComplex32 {
        re: 1.0,
        im: 2.0000036,
    };

    let abs_diff = a.abs_diff(&a);
    assert_eq!(abs_diff.re, 0.0);
    assert_eq!(abs_diff.im, 0.0);

    let ulps_diff = a.ulps_diff(&a);
    assert_eq!(ulps_diff.re, 0);
    assert_eq!(ulps_diff.im, 0);

    let b = MyComplex32 {
        re: 1.0000001,
        im: 2.0,
    };

    let abs_diff = a.abs_diff(&b);
    assert_eq!(abs_diff.re, 0.00000011920929);
    assert_eq!(abs_diff.im, 0.0000035762787);

    let ulps_diff = a.ulps_diff(&b);
    assert_eq!(ulps_diff.re, 1);
    assert_eq!(ulps_diff.im, 15);
}

//------------------------------------------------------------------------------
// FloatDiff with f32
//------------------------------------------------------------------------------
impl FloatDiff<f32> for MyComplex32 {
    type AbsDiff = MyComplex32;
    type UlpsDiff = MyComplex32UlpsDiff;

    fn abs_diff(&self, other: &f32) -> Self::AbsDiff {
        MyComplex32 {
            re: self.re.abs_diff(other),
            im: self.im.abs_diff(&0.0),
        }
    }

    fn ulps_diff(&self, other: &f32) -> Self::UlpsDiff {
        MyComplex32UlpsDiff {
            re: self.re.ulps_diff(other),
            im: self.im.ulps_diff(&0.0),
        }
    }
}

impl FloatDiff<MyComplex32> for f32 {
    type AbsDiff = <MyComplex32 as FloatDiff<f32>>::AbsDiff;
    type UlpsDiff = <MyComplex32 as FloatDiff<f32>>::UlpsDiff;

    fn abs_diff(&self, other: &MyComplex32) -> Self::AbsDiff {
        other.abs_diff(self)
    }

    fn ulps_diff(&self, other: &MyComplex32) -> Self::UlpsDiff {
        other.ulps_diff(self)
    }
}

#[test]
fn float_diff_f32() {
    let a = 1.0000001_f32;
    let b = MyComplex32 {
        re: 1.0,
        im: 2.0000036,
    };

    let abs_diff = a.abs_diff(&b);
    assert_eq!(abs_diff.re, 0.00000011920929);
    assert_eq!(abs_diff.im, 2.0000036);

    let abs_diff = b.abs_diff(&a);
    assert_eq!(abs_diff.re, 0.00000011920929);
    assert_eq!(abs_diff.im, 2.0000036);

    let ulps_diff = a.ulps_diff(&b);
    assert_eq!(ulps_diff.re, 1);
    assert_eq!(ulps_diff.im, 1_073_741_839);

    let ulps_diff = b.ulps_diff(&a);
    assert_eq!(ulps_diff.re, 1);
    assert_eq!(ulps_diff.im, 1_073_741_839);
}

//------------------------------------------------------------------------------
// FloatEq with Self
//------------------------------------------------------------------------------
impl FloatEq for MyComplex32 {
    type DiffEpsilon = f32;
    type UlpsDiffEpsilon = u32;

    fn eq_abs(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
        self.re.eq_abs(&other.re, max_diff) && self.im.eq_abs(&other.im, max_diff)
    }

    fn eq_rel(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> bool {
        self.re.eq_rel(&other.re, max_diff) && self.im.eq_rel(&other.im, max_diff)
    }

    fn eq_ulps(&self, other: &Self, max_diff: &Self::UlpsDiffEpsilon) -> bool {
        self.re.eq_ulps(&other.re, max_diff) && self.im.eq_ulps(&other.im, max_diff)
    }
}

#[test]
fn float_eq_self() {
    let a = MyComplex32 {
        re: 1.0,
        im: 1_000_000.,
    };

    assert!(a.eq_abs(&a, &0.0));
    assert!(a.eq_rel(&a, &0.0));
    assert!(a.eq_ulps(&a, &0));

    let b = MyComplex32 {
        re: 1.0000001,
        im: 1_000_000.,
    };

    assert!(a.ne_abs(&b, &0.0));
    assert!(a.ne_rel(&b, &0.0));
    assert!(a.ne_ulps(&b, &0));

    assert!(a.eq_abs(&b, &0.00000012));
    assert!(a.eq_rel(&b, &0.00000012));
    assert!(a.eq_ulps(&b, &2));

    assert!(b.ne_abs(&a, &0.0));
    assert!(b.ne_rel(&a, &0.0));
    assert!(b.ne_ulps(&a, &0));

    assert!(b.eq_abs(&a, &0.00000012));
    assert!(b.eq_rel(&a, &0.00000012));
    assert!(b.eq_ulps(&a, &2));

    let c = MyComplex32 {
        re: 1.0,
        im: 1_000_000.06,
    };

    assert!(a.ne_abs(&c, &0.0));
    assert!(a.ne_rel(&c, &0.0));
    assert!(a.ne_ulps(&c, &0));

    assert!(a.eq_abs(&c, &0.07));
    assert!(a.eq_rel(&c, &0.0000002));
    assert!(a.eq_ulps(&c, &1));

    assert!(c.ne_abs(&a, &0.0));
    assert!(c.ne_rel(&a, &0.0));
    assert!(c.ne_ulps(&a, &0));

    assert!(c.eq_abs(&a, &0.07));
    assert!(c.eq_rel(&a, &0.0000002));
    assert!(c.eq_ulps(&a, &1));
}

//------------------------------------------------------------------------------
// FloatEq with f32
//------------------------------------------------------------------------------
impl FloatEq<f32> for MyComplex32 {
    type DiffEpsilon = f32;
    type UlpsDiffEpsilon = u32;

    fn eq_abs(&self, other: &f32, max_diff: &Self::DiffEpsilon) -> bool {
        self.re.eq_abs(other, max_diff) && self.im.eq_abs(&0.0, max_diff)
    }

    fn eq_rel(&self, other: &f32, max_diff: &Self::DiffEpsilon) -> bool {
        self.re.eq_rel(other, max_diff) && self.im.eq_rel(&0.0, max_diff)
    }

    fn eq_ulps(&self, other: &f32, max_diff: &Self::UlpsDiffEpsilon) -> bool {
        self.re.eq_ulps(other, max_diff) && self.im.eq_ulps(&0.0, max_diff)
    }
}

impl FloatEq<MyComplex32> for f32 {
    type DiffEpsilon = <MyComplex32 as FloatEq<f32>>::DiffEpsilon;
    type UlpsDiffEpsilon = <MyComplex32 as FloatEq<f32>>::UlpsDiffEpsilon;

    fn eq_abs(&self, other: &MyComplex32, max_diff: &Self::DiffEpsilon) -> bool {
        other.eq_abs(self, max_diff)
    }

    fn eq_rel(&self, other: &MyComplex32, max_diff: &Self::DiffEpsilon) -> bool {
        other.eq_rel(self, max_diff)
    }

    fn eq_ulps(&self, other: &MyComplex32, max_diff: &Self::UlpsDiffEpsilon) -> bool {
        other.eq_ulps(self, max_diff)
    }
}

#[test]
fn float_eq_f32() {
    let a = 1_000_000.06;
    let b = MyComplex32 {
        re: 1_000_000.0,
        im: 2.0,
    };

    assert!(a.ne_abs(&b, &0.07));
    assert!(a.ne_rel(&b, &0.00000012));
    assert!(a.ne_ulps(&b, &1));

    assert!(a.eq_abs(&b, &2.0));
    assert!(a.eq_rel(&b, &2.0));
    assert!(a.eq_ulps(&b, &1_073_741_824));

    assert!(b.ne_abs(&a, &0.07));
    assert!(b.ne_rel(&a, &0.00000012));
    assert!(b.ne_ulps(&a, &1));

    assert!(b.eq_abs(&a, &2.0));
    assert!(b.eq_rel(&a, &2.0));
    assert!(b.eq_ulps(&a, &1_073_741_824));

    let c = 2.0000004;
    let d = MyComplex32 { re: 2.0, im: 0.0 };

    assert!(c.ne_abs(&d, &0.00000004));
    assert!(c.ne_rel(&d, &0.000000023));
    assert!(c.ne_ulps(&d, &1));

    assert!(c.eq_abs(&d, &0.0000005));
    assert!(c.eq_rel(&d, &0.00000024));
    assert!(c.eq_ulps(&d, &2));

    assert!(d.ne_abs(&c, &0.00000004));
    assert!(d.ne_rel(&c, &0.000000023));
    assert!(d.ne_ulps(&c, &1));

    assert!(d.eq_abs(&c, &0.0000005));
    assert!(d.eq_rel(&c, &0.00000024));
    assert!(d.eq_ulps(&c, &2));
}

//------------------------------------------------------------------------------
// FloatEqDebug with Self
//------------------------------------------------------------------------------
impl FloatEqDebug for MyComplex32 {
    type DebugEpsilon = Self;
    type DebugUlpsEpsilon = MyComplex32UlpsDiff;

    fn debug_abs_epsilon(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> Self::DebugEpsilon {
        MyComplex32 {
            re: self.re.debug_abs_epsilon(&other.re, max_diff),
            im: self.im.debug_abs_epsilon(&other.im, max_diff),
        }
    }

    fn debug_rel_epsilon(&self, other: &Self, max_diff: &Self::DiffEpsilon) -> Self::DebugEpsilon {
        MyComplex32 {
            re: self.re.debug_rel_epsilon(&other.re, max_diff),
            im: self.im.debug_rel_epsilon(&other.im, max_diff),
        }
    }

    fn debug_ulps_epsilon(
        &self,
        other: &Self,
        max_diff: &Self::UlpsDiffEpsilon,
    ) -> Self::DebugUlpsEpsilon {
        MyComplex32UlpsDiff {
            re: self.re.debug_ulps_epsilon(&other.re, max_diff),
            im: self.im.debug_ulps_epsilon(&other.im, max_diff),
        }
    }
}

#[test]
fn float_eq_debug_self() {
    let a = MyComplex32 { re: 1.0, im: 200.0 };
    let b = MyComplex32 { re: 50.0, im: 1.0 };

    assert_eq!(
        a.debug_abs_epsilon(&b, &0.1),
        MyComplex32 { re: 0.1, im: 0.1 }
    );
    assert_eq!(
        b.debug_abs_epsilon(&a, &0.1),
        MyComplex32 { re: 0.1, im: 0.1 }
    );

    assert_eq!(
        a.debug_rel_epsilon(&b, &0.1),
        MyComplex32 { re: 5.0, im: 20.0 }
    );
    assert_eq!(
        b.debug_rel_epsilon(&a, &0.1),
        MyComplex32 { re: 5.0, im: 20.0 }
    );

    assert_eq!(
        a.debug_ulps_epsilon(&b, &42),
        MyComplex32UlpsDiff { re: 42, im: 42 }
    );
    assert_eq!(
        b.debug_ulps_epsilon(&a, &42),
        MyComplex32UlpsDiff { re: 42, im: 42 }
    );
}

//------------------------------------------------------------------------------
// FloatEqDebug with f32
//------------------------------------------------------------------------------
impl FloatEqDebug<f32> for MyComplex32 {
    type DebugEpsilon = Self;
    type DebugUlpsEpsilon = MyComplex32UlpsDiff;

    fn debug_abs_epsilon(&self, other: &f32, max_diff: &Self::DiffEpsilon) -> Self::DebugEpsilon {
        MyComplex32 {
            re: self.re.debug_abs_epsilon(other, max_diff),
            im: self.im.debug_abs_epsilon(&0.0, max_diff),
        }
    }

    fn debug_rel_epsilon(&self, other: &f32, max_diff: &Self::DiffEpsilon) -> Self::DebugEpsilon {
        MyComplex32 {
            re: self.re.debug_rel_epsilon(other, max_diff),
            im: self.im.debug_rel_epsilon(&0.0, max_diff),
        }
    }

    fn debug_ulps_epsilon(
        &self,
        other: &f32,
        max_diff: &Self::UlpsDiffEpsilon,
    ) -> Self::DebugUlpsEpsilon {
        MyComplex32UlpsDiff {
            re: self.re.debug_ulps_epsilon(other, max_diff),
            im: self.im.debug_ulps_epsilon(&0.0, max_diff),
        }
    }
}

impl FloatEqDebug<MyComplex32> for f32 {
    type DebugEpsilon = <MyComplex32 as FloatEqDebug<f32>>::DebugEpsilon;
    type DebugUlpsEpsilon = <MyComplex32 as FloatEqDebug<f32>>::DebugUlpsEpsilon;

    fn debug_abs_epsilon(
        &self,
        other: &MyComplex32,
        max_diff: &Self::DiffEpsilon,
    ) -> Self::DebugEpsilon {
        other.debug_abs_epsilon(self, max_diff)
    }

    fn debug_rel_epsilon(
        &self,
        other: &MyComplex32,
        max_diff: &Self::DiffEpsilon,
    ) -> Self::DebugEpsilon {
        other.debug_rel_epsilon(self, max_diff)
    }

    fn debug_ulps_epsilon(
        &self,
        other: &MyComplex32,
        max_diff: &Self::UlpsDiffEpsilon,
    ) -> Self::DebugUlpsEpsilon {
        other.debug_ulps_epsilon(self, max_diff)
    }
}

#[test]
fn float_eq_debug_f32() {
    let a = MyComplex32 {
        re: 150.0,
        im: 200.0,
    };
    let b = 1.0_f32;

    assert_eq!(
        a.debug_abs_epsilon(&b, &0.1),
        MyComplex32 { re: 0.1, im: 0.1 }
    );
    assert_eq!(
        b.debug_abs_epsilon(&a, &0.1),
        MyComplex32 { re: 0.1, im: 0.1 }
    );

    assert_eq!(
        a.debug_rel_epsilon(&b, &0.1),
        MyComplex32 { re: 15.0, im: 20.0 }
    );
    assert_eq!(
        b.debug_rel_epsilon(&a, &0.1),
        MyComplex32 { re: 15.0, im: 20.0 }
    );

    assert_eq!(
        a.debug_ulps_epsilon(&b, &42),
        MyComplex32UlpsDiff { re: 42, im: 42 }
    );
    assert_eq!(
        b.debug_ulps_epsilon(&a, &42),
        MyComplex32UlpsDiff { re: 42, im: 42 }
    );

    let c = 9000.0_f32;

    assert_eq!(
        a.debug_abs_epsilon(&c, &0.1),
        MyComplex32 { re: 0.1, im: 0.1 }
    );
    assert_eq!(
        c.debug_abs_epsilon(&a, &0.1),
        MyComplex32 { re: 0.1, im: 0.1 }
    );

    assert_eq!(
        a.debug_rel_epsilon(&c, &0.1),
        MyComplex32 {
            re: 900.0,
            im: 20.0
        }
    );
    assert_eq!(
        c.debug_rel_epsilon(&a, &0.1),
        MyComplex32 {
            re: 900.0,
            im: 20.0
        }
    );

    assert_eq!(
        a.debug_ulps_epsilon(&c, &42),
        MyComplex32UlpsDiff { re: 42, im: 42 }
    );
    assert_eq!(
        c.debug_ulps_epsilon(&a, &42),
        MyComplex32UlpsDiff { re: 42, im: 42 }
    );
}

// assert_float_eq!/assert_float_ne! with Self
// assert_float_eq!/assert_float_ne! with f32
