#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2{
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }
}

#[derive(Debug, Clone, Copy)]

pub struct Bounds {
    pub top_left: Vec2,
    pub bottom_right: Vec2
}

impl Bounds { 
    pub fn new(top_left: Vec2, bottom_right: Vec2) -> Self {
        Self { top_left, bottom_right }
    }

    pub fn width(&self) -> f64 {
        self.bottom_right.x - self.top_left.x
    }

    pub fn height(&self) -> f64{
        self.top_left.y - self.bottom_right.y
    }

    pub fn rebase_to_canvas(&self, overall_origin: Vec2) -> Self {
        Bounds::new(
            Vec2::new(self.top_left.x - overall_origin.x, self.top_left.y - overall_origin.y),
            Vec2::new(self.bottom_right.x - overall_origin.x, self.bottom_right.y - overall_origin.y),
        )
    }

}

impl TryFrom<&str> for Bounds{
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts : Vec<f64> = s
        .split(",")   
        .map(|v| v.trim().parse::<f64>())
        .collect::<Result<Vec<f64>, _>>()?;

        anyhow::ensure!(parts.len() == 4, "Expected 4 values for bounds, got {}", parts.len());

        Ok(Bounds::new(
            Vec2::new(parts[0], parts[1]),
            Vec2::new(parts[2], parts[3]),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rebase_shifts_correctly() {
        let artboard = Bounds::new(
            Vec2::new(350.0, 450.0),
            Vec2::new(650.0, 550.0),
        );

        let origin = Vec2::new(100.0, 300.0);
        let rebased = artboard.rebase_to_canvas(origin);

        assert_eq!(rebased.top_left, Vec2::new(250.0, 150.0));
        assert_eq!(rebased.bottom_right, Vec2::new(550.0, 250.0));
    }

    #[test]
    fn parse_bounds_from_str() {
        let b = Bounds::try_from("100,200,300,400").unwrap();
        assert_eq!(b.top_left, Vec2::new(100.0, 200.0));
        assert_eq!(b.bottom_right, Vec2::new(300.0, 400.0));
    }
}



