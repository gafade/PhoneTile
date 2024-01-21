use std::vec;

use crate::network::player;
use super::game::FILET;

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
    pub fn new(p: &player::Player,sX:f64) -> Racket {
        Racket {

            id:p.rank as usize,
            pos:0.,
            height:200.,//dynamique, en fonction écran

            speed: 0.,
            
            role: false,
            team:0,

            sizeX:sX,
            sizeY:FILET,
        }
    }

    pub fn update_status(&mut self, absX:f64){//coordonnées virtuelles du doigt du joueur,
        // game s'occupe de convertir en fonction des dimensions du tel




    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    //Envoyer les coorddonnees des rectangles et leur ID et largeur longueur
    pub fn rect(self, p : &player::Player) -> Vec<u8>{

        let mut data = vec::Vec::new();
        data.push((self.id as u8).to_be());

        let (x, y) = p.to_local_coordinates(self.pos as f32,FILET as f32);//on sen fiche selon y : cest un pong
        let pos_x = x.to_be_bytes();
        let pos_y = y.to_be_bytes();
        data.append(&mut pos_x.to_vec());
        data.append(&mut pos_y.to_vec());

        //pour sizeX sizeY
        let size_x = self.sizeX.to_be_bytes();
        let size_y = self.sizeY.to_be_bytes();
        data.append(&mut size_x.to_vec());
        data.append(&mut size_y.to_vec());


        data//[ID, posx[],posy[],sizeX[],sizeY[]]
        
    }
}
