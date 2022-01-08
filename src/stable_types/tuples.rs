use std::fmt::Debug;

#[repr(C)]
pub struct BwsTuple2<T1: Sized, T2>(pub T1, pub T2);

#[repr(C)]
pub struct BwsTuple3<T1: Sized, T2: Sized, T3>(pub T1, pub T2, pub T3);

#[repr(C)]
pub struct BwsTuple4<T1: Sized, T2: Sized, T3: Sized, T4>(pub T1, pub T2, pub T3, pub T4);

#[repr(C)]
pub struct BwsTuple5<T1: Sized, T2: Sized, T3: Sized, T4: Sized, T5>(
    pub T1,
    pub T2,
    pub T3,
    pub T4,
    pub T5,
);

#[repr(C)]
pub struct BwsTuple6<T1: Sized, T2: Sized, T3: Sized, T4: Sized, T5: Sized, T6>(
    pub T1,
    pub T2,
    pub T3,
    pub T4,
    pub T5,
    pub T6,
);

impl<T1: Debug, T2: Debug> Debug for BwsTuple2<T1, T2> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.0, self.1)
    }
}

impl<T1: Debug, T2: Debug, T3: Debug> Debug for BwsTuple3<T1, T2, T3> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.0, self.1, self.2)
    }
}

impl<T1: Debug, T2: Debug, T3: Debug, T4: Debug> Debug for BwsTuple4<T1, T2, T3, T4> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {:?}, {:?}, {:?})",
            self.0, self.1, self.2, self.3
        )
    }
}

impl<T1: Debug, T2: Debug, T3: Debug, T4: Debug, T5: Debug> Debug
    for BwsTuple5<T1, T2, T3, T4, T5>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {:?}, {:?}, {:?}, {:?})",
            self.0, self.1, self.2, self.3, self.4
        )
    }
}

impl<T1: Debug, T2: Debug, T3: Debug, T4: Debug, T5: Debug, T6: Debug> Debug
    for BwsTuple6<T1, T2, T3, T4, T5, T6>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            self.0, self.1, self.2, self.3, self.4, self.5
        )
    }
}

impl<T1: Clone + Sized, T2: Clone> Clone for BwsTuple2<T1, T2> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<T1: Clone + Sized, T2: Clone + Sized, T3: Clone> Clone for BwsTuple3<T1, T2, T3> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), self.2.clone())
    }
}

impl<T1: Clone + Sized, T2: Clone + Sized, T3: Clone + Sized, T4: Clone> Clone
    for BwsTuple4<T1, T2, T3, T4>
{
    fn clone(&self) -> Self {
        Self(
            self.0.clone(),
            self.1.clone(),
            self.2.clone(),
            self.3.clone(),
        )
    }
}

impl<T1: Clone + Sized, T2: Clone + Sized, T3: Clone + Sized, T4: Clone + Sized, T5: Clone> Clone
    for BwsTuple5<T1, T2, T3, T4, T5>
{
    fn clone(&self) -> Self {
        Self(
            self.0.clone(),
            self.1.clone(),
            self.2.clone(),
            self.3.clone(),
            self.4.clone(),
        )
    }
}

impl<
        T1: Clone + Sized,
        T2: Clone + Sized,
        T3: Clone + Sized,
        T4: Clone + Sized,
        T5: Clone + Sized,
        T6: Clone,
    > Clone for BwsTuple6<T1, T2, T3, T4, T5, T6>
{
    fn clone(&self) -> Self {
        Self(
            self.0.clone(),
            self.1.clone(),
            self.2.clone(),
            self.3.clone(),
            self.4.clone(),
            self.5.clone(),
        )
    }
}
