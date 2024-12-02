#[derive(Debug,Clone, Copy)]
pub struct Viewport{
    x:i32,
    y:i32,
    width:i32,
    height:i32
}
impl Viewport{
    pub fn new(x:i32,y:i32,width:i32,height:i32) -> Self{
        Self { x, y, width, height }
    }
    pub fn set_pos(&mut self,x:i32,y:i32){
        self.x = x;
        self.y = y;
    }
    pub fn width(&self) -> i32{
        self.width
    }
    pub fn height(&self) -> i32{
        self.height
    }
    pub fn x(&self) -> i32{
        self.x
    }
    pub fn y(&self) -> i32{
        self.y
    }
    pub fn set_size(&mut self,width:i32,height:i32){
        self.width = width;
        self.height = height;
    }
    /// binding viewport to opengl context 
    pub fn set_gl_viewport(&self){
        unsafe{
            gl::Viewport(self.x, self.y, self.width, self.height);
        }
    }
}
impl Default for Viewport {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), width: Default::default(), height: Default::default() }
    }
}