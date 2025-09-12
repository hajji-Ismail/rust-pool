pub enum GeometricalShapes {
    Square,
    Circle,
    Rectangle,
    Triangle,
}

pub enum GeometricalVolumes {
    Cube,
    Sphere,
    Cone,
    TriangularPyramid,
    Parallelepiped,
}

pub(crate) fn square_area(side: usize) -> usize {
    side.pow(2)
}

pub(crate) fn triangle_area(base: usize, height: usize) -> f64 {
    (base as f64 * height as f64) / 2.0
}

pub(crate) fn circle_area(radius: usize) -> f64 {
    std::f64::consts::PI * (radius.pow(2) as f64)
}

pub(crate) fn rectangle_area(side_a: usize, side_b: usize) -> usize {
    side_a * side_b
}

pub(crate) fn cube_volume(side: usize) -> usize {
    side.pow(3)
}

pub(crate) fn sphere_volume(radius: usize) -> f64 {
    (4.0 / 3.0) * std::f64::consts::PI * (radius.pow(3) as f64)
}

pub(crate) fn triangular_pyramid_volume(base_area: f64, height: usize) -> f64 {
    (base_area * height as f64) / 3.0
}

pub(crate) fn parallelepiped_volume(side_a: usize, side_b: usize, side_c: usize) -> usize {
    side_a * side_b * side_c
}

pub(crate) fn cone_volume(base_radius: usize, height: usize) -> f64 {
    (1.0 / 3.0) * std::f64::consts::PI * base_radius.pow(2) as f64 * height as f64
}

pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let erea = x*y; 
    let mut res = 0 ;
    match kind {
        GeometricalShapes::Circle => {
           res = circle_area(a)  as usize;

        },
        GeometricalShapes::Square => {
            res = square_area(a) ;
        },
        GeometricalShapes::Rectangle => {
            res = rectangle_area(a, b) ;
        } ,
        GeometricalShapes::Triangle => {
            res = triangle_area(a, b) as usize;
        }
    }
    if erea >= res * times{
        return  true;

    }
false
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
        let erea = (x*y*z) as f64 ; 
    let mut res = 0.0 ;
    match kind {
        GeometricalVolumes::TriangularPyramid => {
          res =   triangular_pyramid_volume(a as f64, b);
         

        },
        GeometricalVolumes::Cone => {
            res = cone_volume(a, b);
        },
        GeometricalVolumes::Cube => {
            res = (cube_volume(a )) as f64
        },
        GeometricalVolumes::Parallelepiped => {
            res =( parallelepiped_volume(a, b, c)) as f64 ;
        },
        GeometricalVolumes::Sphere => {
            res = ( sphere_volume(a)) as f64 ;
        }
    
    }
    if erea >= res * times as f64 {
        return  true;

    }
false

  
}