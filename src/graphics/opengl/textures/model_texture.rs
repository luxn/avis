pub struct ModelTexture {
    texture_id: u32,
}


impl ModelTexture {

    pub fn new(id: u32) -> Self {
        ModelTexture {
            texture_id: id
        }
    }


    pub fn id(&self) -> u32 {
        self.texture_id
    }

}