mod areas_volumes;
pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let erea = (x*y) as f64 ; 
    let mut res = 0.0 ;
    match kind {
        areas_volumes::GeometricalShapes::Circle => {
           res = areas_volumes::circle_area(a);

        },
        areas_volumes::GeometricalShapes::Square => {
            res = (areas_volumes::square_area(a)) as f64 ;
        },
        areas_volumes::GeometricalShapes::Rectangle => {
            res = areas_volumes::rectangle_area(a, b) as f64;
        } ,
        areas_volumes::GeometricalShapes::Triangle => {
            res = areas_volumes::triangle_area(a, b) 
        }
    }
    if erea == res * times as f64 {
        return  true;

    }
false
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
        let erea = (x*y*z) as f64 ; 
    let mut res = 0.0 ;
    match kind {
        areas_volumes::GeometricalVolumes::TriangularPyramid => {
          res =   areas_volumes::triangular_pyramid_volume(a as f64, b);
         

        },
        areas_volumes::GeometricalVolumes::Cone => {
            res = areas_volumes::cone_volume(a, b);
        },
        areas_volumes::GeometricalVolumes::Cube => {
            res = (areas_volumes::cube_volume(a )) as f64
        },
        areas_volumes::GeometricalVolumes::Parallelepiped => {
            res =( areas_volumes::parallelepiped_volume(a, b, c)) as f64 ;
        },
        areas_volumes::GeometricalVolumes::Sphere => {
            res = ( areas_volumes::sphere_volume(a)) as f64 ;
        }
    
    }
    if erea == res * times as f64 {
        return  true;

    }
false

  
}