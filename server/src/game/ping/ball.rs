use std::time;

use super::racket::Racket;

#[derive(Debug, Clone)]


pub struct Ball {
    pub posX: f64,//position sur la map, et pas sur les téléphones
    pub posY:f64,


    pub speedX: f64,
    pub speedY:f64,

    pub state:f32,//en cours de creation/ en mouvement/ dans le but/fin du jeu

    size:f64,//uniquement visuel/pas distordu par la taille des écrans
   
}

impl Ball {

    pub fn new(X:f64,Y:f64,s:f64) -> Ball {
        Ball {
            posX:X,
            posY: Y,
            
            speedX:0.,
            speedY:0.,

            state:0.,

            size:s,
        }

        
    }

pub fn update_status(&mut self, 
    players : Vec<Racket> ,
    internal_timer: time::Instant,
) {
    //Gestion mouvement
    self.posX += self.speedX * internal_timer.elapsed().as_secs_f64() * 50.;
    self.posY += self.speedY * internal_timer.elapsed().as_secs_f64() * 50.;


    //Gestion bords terrain (et pas de l'écran)
    if self.posX < 0. {
        self.speedX = self.speedX.abs()+1.;}
    if self.posX > 500. {// TODO Xmax
        self.speedX = -self.speedX.abs()-1.;}


        if self.posY < 0.{
            self.speedY=self.speedY.abs()+1.;}
        if self.speedY > 5000.{// TODO Ymax
            self.speedY= - self.speedY.abs() - 1.;}

        //Gestion collision avec players
        for p in players{
            if (self.posX-p.pos).abs()<p.sizeX &&
            (self.posY-p.height).abs()<p.sizeY{
                //empecher clipping/multi rebonds

            }
        }

        //Gestion collision avec un goal : player en bout piste=> vitesse augmente?
        //ou les goals sont les seuls qui peuvent mettre un effet?

        //Gestion arrivée dans un but?
        //non la liste de ball s'occupera de les sortir du jeu

        //Collision avec une balle

    }

    pub fn exit(//la flemme et destructeur : on la pose en dehors terrain
        &mut self
    ){
        self.speedX=0.;
        self.speedY=0.;

        self.posX=-100.;
        self.posY=-100.;

    }

    pub fn enter(&mut self,Speed:f64,dir :i8 ){//pour commencer, seulement envoyées en diagonale


        //TODO : trouver centre du terrain
        self.posX=250.;
        self.posY=2500.;

        if(dir%4==0){
            self.speedX=Speed;
            self.speedY=Speed;
                }
        if(dir%4==1){
            self.speedX=Speed;
            self.speedY=-Speed;
                }
        if(dir%4==2){
            self.speedX=-Speed;
            self.speedY=Speed;
                }
        if(dir%4==3){
            self.speedX=-Speed;
            self.speedY=-Speed;
                        }
        
    }
}
