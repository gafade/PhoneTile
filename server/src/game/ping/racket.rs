use std::vec;

use crate::network::player;
use super::game::FILET;
use super::game::LARGEUR;
use super::game::LONGUEUR;

#[derive(Debug, Clone)]
pub struct Racket {

    //Communication
    pub id:usize,

    pub pos: f64,
    pub height:f64,// constant
    pub speed: f64,//pour les effetts

    pub role:bool,
    pub team: i8,
   
   pub sizeX:f64,
   pub sizeY:f64,
}

impl Racket {
    pub fn new(p: &player::Player,sX:f64,position:f64) -> Racket {
        let place:u8=  p.rank;
        if (place%2==0){//equipe Pair
        Racket {

            id:p.rank as usize,
            pos:0.,
            //NON LA VRAI POS SUR LE TERRAIN
            height:position+500.*(place/2) as f64,
            

            speed: 0.,
            
            role: false,
            team:0,

            sizeX:sX,
            sizeY:FILET,
        }
    }else{//equipe impaire
        Racket {

            id:p.rank as usize,
            pos:0.,
            //NON LA VRAI POS SUR LE TERRAIN
            height:5000.-position-500.*((place-1)/2) as f64,
            

            speed: 0.,
            
            role: false,
            team:0,

            sizeX:sX,
            sizeY:FILET,
        }
    }
    }

    pub fn update_status(&mut self, abs:f64){//coordonnées virtuelles du doigt du joueur,
        // game s'occupe de convertir en fonction des dimensions du tel

        self.pos=abs as f64;



    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    //Envoyer les coorddonnees des rectangles et leur ID et largeur longueur
    pub fn rect(&self, p : &player::Player) -> Vec<u8>{

        let mut data = vec::Vec::new();
        data.push((self.id as u8).to_be());

        let mut x:f32;
        let mut y:f32; 
        let mut sX:f32;
        let mut sY:f32;

        //coord server -> coord physical
        let factorX=p.physical_width/LONGUEUR as f32;
        let factorY=p.physical_height/LARGEUR as f32;
        if self.id%2==0{
            x=self.height as f32-500.*(p.rank/2) as f32;
            y=self.pos as f32;// - size/2 pour centrer sur le doigt?
        }else{//inverser pour raquette en bas
            x=5000.-self.height as f32-500.*((p.rank-1)/2) as f32;
            y=self.pos as f32;
        }
        (x,y)=p.to_local_coordinates(x*factorX,y*factorY);

        let pos_x = x.to_be_bytes();
        let pos_y = y.to_be_bytes();
        data.append(&mut pos_x.to_vec());
        data.append(&mut pos_y.to_vec());

        //pour sizeX sizeY, inutile
        (sX,sY)=p.to_local_coordinates(self.sizeX as f32, self.sizeY as f32);
        let size_x = sX.to_be_bytes();
        let size_y = sY.to_be_bytes();
        data.append(&mut size_x.to_vec());
        data.append(&mut size_y.to_vec());


        data//[ID, posx[],posy[],sizeX[],sizeY[]]
        
    }
}
