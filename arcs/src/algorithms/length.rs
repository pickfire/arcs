use crate::{
    primitives::{Arc, Line},
    Vector,
};

/// Something which has a finite length.
pub trait Length {
    /// Calculate the length.
    fn length(&self) -> f64;
}

impl Length for Line {
    /// Calculates the length of the line.
    ///
    /// ```rust
    /// # use arcs::{algorithms::Length, primitives::Line, Vector};
    /// let line = Line::new(Vector::zero(), Vector::new(5.0, 0.0));
    ///
    /// assert_eq!(line.length(), 5.0);
    /// ```
    fn length(&self) -> f64 { self.displacement().length() }
}

impl Length for Vector {
    /// Calculates the [`Vector`]'s magnitude.
    ///
    /// ```rust
    /// # use arcs::{algorithms::Length, Vector};
    /// let vector = Vector::new(3.0, 4.0);
    ///
    /// assert_eq!(vector.length(), 5.0);
    /// ```
    fn length(&self) -> f64 { Vector::length(*self) }
}

impl Length for Arc {
    /// Calculates the length of an [`Arc`].
    ///
    /// ```rust
    /// # use arcs::{algorithms::Length, primitives::Arc, Vector};
    /// # use std::f64::consts::PI;
    /// let radius = 50.0;
    /// let arc = Arc::from_centre_radius(Vector::zero(), radius, 0.0, 2.0 * PI);
    ///
    /// assert_eq!(arc.length(), 2.0 * radius * PI);
    /// ```
    fn length(&self) -> f64 { self.radius() * self.sweep_angle().abs() }
}
