use crate::network::packet;
use crate::network::{self, player};
use log::info;
use rand::{self, random};
use std::io::Error;
use std::thread;
use std::time;
use std::vec;



use self::game::{get_screen, LONGUEUR,LARGEUR};

//////////////////////////////////////////////
///
///
/// Modules
///
///
//////////////////////////////////////////////
mod ball;
mod game;
mod racket;

//////////////////////////////////////////////
///
///
/// Constants
///
///
//////////////////////////////////////////////

const BALL_SIZE: usize = 20;
const INIT_BALL_SPEED:f64= 4.;
const SPAWN_SPEED:u16= 200;

const RACKET_SIZE:f64=50.;
//////////////////////////////////////////////
///
///
/// Entry point/Game loop thread
///
///
//////////////////////////////////////////////

pub fn ping_loop(players: &mut [network::player::Player]){

    //update les balles actives+ les desactiver si en dehors de l'ecran

    //INIT

    info!(target: "ping_loop", "Jeu demarré");

    let mut width: Vec<f32> = vec![];
    let mut height: Vec<f32> = vec![];
    (width,height)=get_screen(players);

    let mut score:(u8,u8)=(0,0);

    let mut Active: Vec<ball::Ball> = vec::Vec::new();//un autre thread les genere periodiquement avec direcion random
    let mut Off:Vec<ball::Ball> = vec::Vec::new();

    let mut internal_timer = time::Instant::now();

    let mut last_modifier_gen = time::Instant::now();

    let mut rackets:Vec<racket::Racket> = vec::Vec::new();
    for p in players.iter() {
        //pb d'argumetns, sX sert à rien, sY est la pos importante
        rackets.push(racket::Racket::new(p,RACKET_SIZE,300.));//250 EST UNE CONSTANTE ENTRE SERVER ET CLIENT
        //VOIR client>ping>mod.rs ligne ...
        }

    //arrivée de nouvelles balles
    let mut spawn:u16=0;
    loop {

        //for r in rackets.iter_mut() {
            //recvei game data s'occupe d'update leurs valeurs?
        //}
        //enlever les balles au but
        for index_b in 0..Active.len(){
            Active[index_b].update_status(&mut rackets, internal_timer);
            if Active[index_b].posX>0.95*LONGUEUR {
                Off.push(Active.swap_remove(index_b));//changer de liste la balle Active->off
                score.0+=1;
                break;//sinon si une ballle est remove pendant la boucle, OutofRange error
            }
            if Active[index_b].posX<0.05*LONGUEUR{
                Off.push(Active.swap_remove(index_b));//changer de liste la balle Active->off
                score.1+=1;
                break;
            }
        }

        internal_timer = time::Instant::now();
        
        //Arrivé des nouveles balles
        spawn+=1;//ou spawn+score pour spawn de + en + vite??
        if spawn>SPAWN_SPEED{
            info!(target : "ping_loop","nouvelle balle");
            let mut newball=ball::Ball::new(0.,0.,BALL_SIZE as f64);
            newball.enter(INIT_BALL_SPEED,random::<u8>());
            Active.push(newball);
            spawn=0;
        }
        //communication et update position des rackets
        for p in players.iter_mut() {
            send_game_data(p, &Active,rackets.as_slice());
            recv_game_data(p, &mut rackets);
        }
        thread::sleep(time::Duration::from_millis(10));
    }

}




//AFFICHAGE COMMUNICATION WITH PLAYERS


fn send_game_data(
    p: &mut player::Player,
    balls: &[ball::Ball],//liste de ballles ACTIVES
    rackets: &[racket::Racket],//juste la racket du jouer concerné?
    //on fait unidirectionnel et le tel affiche temps réel et server recup x,y et decide si rebond ou pas?
) -> Result<(), Error> {
    let mut data = vec::Vec::new();

    //personne ne devrait voir plusieurs rackets
    for r in rackets.iter() {
        if r.get_id() == p.rank as usize { 
            //ca devrait etre les coordonnées affichées? ou on affiche sur les tels sur la pos du doigt
            // cad mentir au joueur, deplacer raacket elors que le serveur a pas encore recu??
            data.append(&mut r.rect(p));

        }
    }
    //pour l'instatn, on envoie toutes les balles actives, pas celles qui sont visibles du joueur
    data.push((balls.len() as u8).to_be());

    for b in balls.iter() {
        //CONTRAIREMENT A MAZE , on envoie pas la vitesse de la balle , les telephones ne vont pas devine la pos des balles
        data.append(&mut b.circle(p));
    }
    p.send(&data)
    //1*racket : ID, posx[],posy[],sizeX[],sizeY[] | u8,?,?,?,?,?
    //n                                            | u8
    //n*ball : //posx[],posy[],radius[]            | u8,u8,u8
}



fn recv_game_data(p: &mut player::Player,
     rackets: &mut [racket::Racket]
    ) {
        //???? aucune idée ce qui se passe/ Comment coder le tel
        //ON VEUTchaque tel renvoie uniquement les coord selon X de leurs raquettes
///////////////
    let mut buffer = [0_u8; packet::MAX_DATA_SIZE];
    let mut anex = [0_u8; packet::MAX_DATA_SIZE];
    let n1 = p.recv(&mut anex).unwrap();
    buffer.copy_from_slice(&anex);
    let mut n = p.recv(&mut anex).unwrap();
    while n > 0 {
        buffer.copy_from_slice(&anex);
        n = p.recv(&mut anex).unwrap();
    }
///////////


    if n1 > 0 {
        for r in rackets.iter_mut() {
            if r.get_id() == p.rank as usize {
//////////////////////////
/// ON VEUT update la valeur de pos de racket
                let mut bb = [0_u8; 4];

                bb.copy_from_slice(&buffer[..4]);
                let received=f32::from_be_bytes(bb);
                r.pos=LARGEUR*(received/p.window_height as f32) as f64;
                info!(target : "Racket","Position actuelle{}",received);
                //s.speed.x = f32::from_be_bytes(bb);
            }
        }
//////////////////////////
    }
}
