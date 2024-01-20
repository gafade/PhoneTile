#[derive(Debug, Clone)]
pub struct Racket {
    pub pos: f64,
    pub height:f64,// constant
    pub speed: f64,//pour les effetts

    pub role:bool,
    pub team: i8,
   
   pub sizeX:f64,
   pub sizeY: f64,
}

impl Racket {
    pub fn new(sX:f64,sY:f64) -> Racket {
        Racket {
            pos:0.,
            height:200.,//dynamique, en fonction écran

            speed: 0.,
            
            role: false,
            team:0,

            sizeX:sX,
            sizeY:sY,
        }
    }

    pub fn update_status(&mut self, absX:f64){//coordonnées virtuelles du doigt du joueur,
        // game s'occupe de convertir en fonction des dimensions du tel



    }
}
