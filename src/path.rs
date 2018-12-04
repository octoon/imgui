use octoon_math::float2;

#[derive(Debug, Clone)]
pub struct Path{
    points: Vec<float2>
}

impl Path{
    pub fn new() -> Path{
        Path{
            points:Vec::new()
        }
    }

    pub fn path_clear(&mut self) -> &mut Self{
        self.points.truncate(0);
        self
    }

    pub fn path_line_to(&mut self, pos:&float2) -> &mut Self{
        self.points.push(pos.clone());
        self
    }

    pub fn path_stoke(&mut self, color:u32, closed:bool, thickness:f32) -> &mut Self{
        self
    }

    pub fn path_rect(&mut self, rect_min:&float2, rect_max:&float2, rounding: f32) -> &mut Self{
        self.path_line_to(rect_min);
        self.path_line_to(&float2::new(rect_max.x, rect_min.y));
        self.path_line_to(rect_max);
        self.path_line_to(&float2::new(rect_min.x, rect_max.y));
        self
    }
}

