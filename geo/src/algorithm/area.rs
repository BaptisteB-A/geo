use num_traits::Float;
use {Line, LineString, MultiPolygon, Polygon, Rect, Triangle, Ring};
use std::iter::Sum;

use algorithm::winding_order::twice_signed_ring_area;

/// Calculation of the area.

pub trait Area<T>
where
    T: Float,
{
    /// Signed area of a geometry.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo::{Coordinate, Point, LineString, Polygon};
    /// use geo::algorithm::area::Area;
    ///
    /// let mut polygon = Polygon::new(LineString::from(vec![
    ///     (0., 0.),
    ///     (5., 0.),
    ///     (5., 6.),
    ///     (0., 6.),
    ///     (0., 0.)
    /// ]), vec![]);
    ///
    /// assert_eq!(polygon.area(), 30.);
    ///
    /// polygon.exterior.0.reverse();
    ///
    /// assert_eq!(polygon.area(), -30.);
    /// ```
    fn area(&self) -> T;
}

impl<T> Area<T> for Ring<T>
where
    T: Float + Sum,
{
    fn area(&self) -> T {
        self.lines().map(|l| l.determinant()).sum()
    }
}

fn get_linestring_area<T>(linestring: &LineString<T>) -> T
where
    T: Float,
{
    twice_signed_ring_area(linestring) / (T::one() + T::one())
}

impl<T> Area<T> for Line<T>
where
    T: Float,
{
    fn area(&self) -> T {
        T::zero()
    }
}

impl<T> Area<T> for Polygon<T>
where
    T: Float + Sum,
{
    fn area(&self) -> T {
        self.exterior.area() - self.interiors.iter().map(|i| i.area()).sum()
    }
}

impl<T> Area<T> for MultiPolygon<T>
where
    T: Float + Sum,
{
    fn area(&self) -> T {
        self.0.iter().map(|p| p.area()).sum()
    }
}

impl<T> Area<T> for Rect<T>
where
    T: Float,
{
    fn area(&self) -> T {
        (self.max.x - self.min.x) * (self.max.y - self.min.y)
    }
}

impl<T> Area<T> for Triangle<T>
where
    T: Float,
{
    fn area(&self) -> T {
        (Line::new(self.0, self.1).determinant()
            + Line::new(self.1, self.2).determinant()
            + Line::new(self.2, self.0).determinant()) / (T::one() + T::one())
    }
}

#[cfg(test)]
mod test {
    use algorithm::area::Area;
    use {Coordinate, Line, LineString, MultiPolygon, Polygon, Rect, Triangle};

    // Area of the polygon
    #[test]
    fn area_empty_polygon_test() {
        let poly = Polygon::<f64>::new(LineString(Vec::new()), Vec::new());
        assert_relative_eq!(poly.area(), 0.);
    }

    #[test]
    fn area_one_point_polygon_test() {
        let poly = Polygon::new(LineString::from(vec![(1., 0.)]), Vec::new());
        assert_relative_eq!(poly.area(), 0.);
    }
    #[test]
    fn area_polygon_test() {
        let linestring = LineString::from(vec![(0., 0.), (5., 0.), (5., 6.), (0., 6.), (0., 0.)]);
        let poly = Polygon::new(linestring, Vec::new());
        assert_relative_eq!(poly.area(), 30.);
    }
    #[test]
    fn rectangle_test() {
        let rect = Rect {
            min: Coordinate { x: 10., y: 30. },
            max: Coordinate { x: 20., y: 40. },
        };
        assert_relative_eq!(rect.area(), 100.);
    }
    #[test]
    fn area_polygon_inner_test() {
        let outer = LineString::from(vec![(0., 0.), (10., 0.), (10., 10.), (0., 10.), (0., 0.)]);
        let inner0 = LineString::from(vec![(1., 1.), (2., 1.), (2., 2.), (1., 2.), (1., 1.)]);
        let inner1 = LineString::from(vec![(5., 5.), (6., 5.), (6., 6.), (5., 6.), (5., 5.)]);
        let poly = Polygon::new(outer, vec![inner0, inner1]);
        assert_relative_eq!(poly.area(), 98.);
    }
    #[test]
    fn area_multipolygon_test() {
        let poly0 = Polygon::new(
            LineString::from(vec![(0., 0.), (10., 0.), (10., 10.), (0., 10.), (0., 0.)]),
            Vec::new(),
        );
        let poly1 = Polygon::new(
            LineString::from(vec![(1., 1.), (2., 1.), (2., 2.), (1., 2.), (1., 1.)]),
            Vec::new(),
        );
        let poly2 = Polygon::new(
            LineString::from(vec![(5., 5.), (6., 5.), (6., 6.), (5., 6.), (5., 5.)]),
            Vec::new(),
        );
        let mpoly = MultiPolygon(vec![poly0, poly1, poly2]);
        assert_eq!(mpoly.area(), 102.);
        assert_relative_eq!(mpoly.area(), 102.);
    }
    #[test]
    fn area_line_test() {
        let line1 = Line::new(Coordinate { x: 0.0, y: 0.0 }, Coordinate { x: 1.0, y: 1.0 });
        assert_eq!(line1.area(), 0.);
    }

    #[test]
    fn area_triangle_test() {
        let triangle = Triangle(
            Coordinate { x: 0.0, y: 0.0 },
            Coordinate { x: 1.0, y: 0.0 },
            Coordinate { x: 0.0, y: 1.0 },
        );
        assert_eq!(triangle.area(), 0.5);

        let triangle = Triangle(
            Coordinate { x: 0.0, y: 0.0 },
            Coordinate { x: 0.0, y: 1.0 },
            Coordinate { x: 1.0, y: 0.0 },
        );
        assert_eq!(triangle.area(), -0.5);
    }
}
