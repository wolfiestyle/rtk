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

        impl std::ops::Div<$elem> for $type {
            type Output = Self;

            #[inline]
            fn div(self, rhs: $elem) -> Self::Output {
                self.map(|a| a / rhs)
            }
        }

        impl std::ops::Rem<$elem> for $type {
            type Output = Self;

            #[inline]
            fn rem(self, rhs: $elem) -> Self::Output {
                self.map(|a| a % rhs)
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

        impl std::ops::DivAssign<$elem> for $type {
            #[inline]
            fn div_assign(&mut self, rhs: $elem) {
                self.map_mut(|a| *a /= rhs)
            }
        }

        impl std::ops::RemAssign<$elem> for $type {
            #[inline]
            fn rem_assign(&mut self, rhs: $elem) {
                self.map_mut(|a| *a %= rhs)
            }
        }
    };
}
