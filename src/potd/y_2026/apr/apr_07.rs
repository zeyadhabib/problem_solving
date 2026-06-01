use std::collections::HashMap;

struct Robot {
    moved: bool,
    idx: usize,
    pos: Vec<(i32, i32)>,
    dir: Vec<i32>,
    to_dir: HashMap<i32, String>,
}

impl Robot {
    fn new(width: i32, height: i32) -> Self {
        let mut pos = Vec::new();
        let mut dir = Vec::new();
        let mut to_dir = HashMap::new();
        
        to_dir.insert(0, "East".to_string());
        to_dir.insert(1, "North".to_string());
        to_dir.insert(2, "West".to_string());
        to_dir.insert(3, "South".to_string());
        
        for i in 0..width {
            pos.push((i, 0));
            dir.push(0);
        }
        for i in 1..height {
            pos.push((width - 1, i));
            dir.push(1);
        }
        for i in (0..=width-2).rev() {
            pos.push((i, height - 1));
            dir.push(2);
        }
        for i in (1..=height-2).rev() {
            pos.push((0, i));
            dir.push(3);
        }
        dir[0] = 3;
        
        Robot {
            moved: false,
            idx: 0,
            pos,
            dir,
            to_dir,
        }
    }
    
    fn step(&mut self, num: i32) {
        self.moved = true;
        self.idx = (self.idx + num as usize) % self.pos.len();
    }
    
    fn get_pos(&self) -> Vec<i32> {
        vec![self.pos[self.idx].0, self.pos[self.idx].1]
    }
    
    fn get_dir(&self) -> String {
        if !self.moved {
            return "East".to_string();
        }
        self.to_dir.get(&self.dir[self.idx]).unwrap().clone()
    }
}