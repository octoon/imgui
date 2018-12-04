use std::any::Any;
use octoon_math::float2;

#[derive(Debug, Clone)]
pub struct DrawVert{
    pos:float2,
    uv:float2,
    color:u32,
}

impl DrawVert{
    pub fn new() -> DrawVert{
        pos:float2::new(0.0, 0.0),
        uv:float2::new(0.0, 0.0),
        color:0
    }
}

pub type DrawIdx = u16;

#[derive(Debug, Clone)]
struct DrawCmd{
    element_count:u32,
    clip_rect:float4,
    texture_id:Any,
    user_callback:fn(),
    user_callback_data:Any,
}

#[derive(Debug, Clone)]
pub struct DrawList{
    cmd_buffer:Vec<DrawCmd>,
    vert_buffer:Vec<DrawVert>,
    idx_buffer:Vec<DrawIdx>,
    
}

impl DrawList{
    /*
    pub fn new() -> DrawList{
        DrawList{
            cmd_buffer:Vec::new(),
            vert_buffer:Vec::new(),
            idx_buffer:Vec::new()
        }
    }
    */

    pub reserve(&mut self, idx_count:i32, vtx_count:i32){
        if let Some(cmd) = self.cmd_buffer.last(){
            cmd.element_count += idx_count;

            let vtx_buffer_old_size = self.vert_buffer.len();
            self.vert_buffer.resize(vtx_buffer_old_size + vtx_count, DrawVert::new());

            let idx_buffer_old_size = self.idx_buffer.len();
            self.idx_buffer.resize(idx_buffer_old_size + idx_count, 0);
        }
    }

    pub fn add_line(&mut self, a:&float2, b:&float2, color:u32, thickness:f32) -> &mut Self{
        self
    }


    pub fn add_rect(&mut self) -> &mut Self{
        self
    }

    pub fn add_convex_poly_filled(&mut self, points:&[float2], color:u32) -> &mut Self{
        let mut path = Path::new();
        let points_count = points.len();
        let idx_count = (points_count-2)*3;
        let vtx_count = points_count;

        if let Some(cmd) = self.cmd_buffer.last(){
            cmd.element_count += idx_count;
        }

        let vtx_buffer_old_size = self.vert_buffer.len();
        self.vert_buffer.resize(vtx_buffer_old_size + vtx_count, DrawVert::new());

        let idx_buffer_old_size = self.idx_buffer.len();
        self.idx_buffer.resize(idx_buffer_old_size + idx_count, 0);
        
        for i in 0..vtx_count{
            self.vert_buffer[vtx_buffer_old_size + i] = DrawVert{
                pos:points[i],
                uv:float2::new(0,0),
                color:color
            };
        }

        let mut writer_idx = idx_buffer_old_size;
        for i in 2..points_count{
            self.idx_buffer[writer_idx] = vtx_buffer_old_size;
            self.idx_buffer[writer_idx + 1] = vtx_buffer_old_size + i - 1;
            self.idx_buffer[writer_idx + 2] = vtx_buffer_old_size + i;
            writer_idx += 3;
        }
        self
    }

}

#[derive(Debug, Clone)]
pub struct Context{
    
}