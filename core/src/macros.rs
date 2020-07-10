macro_rules! implement_map {
    ($elem:ty, $($field:ident),*) => {
        #[inline]
        pub fn map<F>(self, mut f: F) -> Self
        where
            F: FnMut($elem) -> $elem,
        {
            Self {
                $($field: f(self.$field),)*
            }
        }

        #[inline]
        pub fn map2<F>(self, other: Self, mut f: F) -> Self
        where
            F: FnMut($elem, $elem) -> $elem,
        {
            Self {
                $($field: f(self.$field, other.$field),)*
            }
        }

        #[inline]
        pub fn map_mut<F>(&mut self, mut f: F)
        where
            F: FnMut(&mut $elem),
        {
            $(f(&mut self.$field);)*
        }

        #[inline]
        pub fn map2_mut<F>(&mut self, other: Self, mut f: F)
        where
            F: FnMut(&mut $elem, $elem),
        {
            $(f(&mut self.$field, other.$field);)*
        }
    };
}

macro_rules! implement_ops {
    ($type:ty, $elem:ty) => {
        impl std::ops::Add for $type {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                self.map2(rhs, std::ops::Add::add)
            }
        }

        impl std::ops::Sub for $type {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                self.map2(rhs, std::ops::Sub::sub)
            }
        }

        impl std::ops::Mul<$elem> for $type {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: $elem) -> Self::Output {
                self.map(|a| a * rhs)
            }
        }

        impl std::ops::Mul for $type {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self::Output {
                self.map2(rhs, std::ops::Mul::mul)
            }
        }

        impl std::ops::Div<$elem> for $type {
            type Output = Self;

            #[inline]
            fn div(self, rhs: $elem) -> Self::Output {
                self.map(|a| a / rhs)
            }
        }

        impl std::ops::Div for $type {
            type Output = Self;

            #[inline]
            fn div(self, rhs: Self) -> Self::Output {
                self.map2(rhs, std::ops::Div::div)
            }
        }

        impl std::ops::Rem<$elem> for $type {
            type Output = Self;

            #[inline]
            fn rem(self, rhs: $elem) -> Self::Output {
                self.map(|a| a % rhs)
            }
        }

        impl std::ops::Rem for $type {
            type Output = Self;

            #[inline]
            fn rem(self, rhs: Self) -> Self::Output {
                self.map2(rhs, std::ops::Rem::rem)
            }
        }

        impl std::ops::AddAssign for $type {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.map2_mut(rhs, std::ops::AddAssign::add_assign)
            }
        }

        impl std::ops::SubAssign for $type {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.map2_mut(rhs, std::ops::SubAssign::sub_assign)
            }
        }

        impl std::ops::MulAssign<$elem> for $type {
            #[inline]
            fn mul_assign(&mut self, rhs: $elem) {
                self.map_mut(|a| *a *= rhs)
            }
        }

        impl std::ops::MulAssign for $type {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                self.map2_mut(rhs, std::ops::MulAssign::mul_assign)
            }
        }

        impl std::ops::DivAssign<$elem> for $type {
            #[inline]
            fn div_assign(&mut self, rhs: $elem) {
                self.map_mut(|a| *a /= rhs)
            }
        }

        impl std::ops::DivAssign for $type {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                self.map2_mut(rhs, std::ops::DivAssign::div_assign)
            }
        }

        impl std::ops::RemAssign<$elem> for $type {
            #[inline]
            fn rem_assign(&mut self, rhs: $elem) {
                self.map_mut(|a| *a %= rhs)
            }
        }

        impl std::ops::RemAssign for $type {
            #[inline]
            fn rem_assign(&mut self, rhs: Self) {
                self.map2_mut(rhs, std::ops::RemAssign::rem_assign)
            }
        }
    };
}
