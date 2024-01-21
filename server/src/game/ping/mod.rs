use crate::network::packet;
use crate::network::{self, player};
use rand;
use std::io::Error;
use std::slice::range;
use std::thread;
use std::time;
use std::vec;



use self::game::{get_screen, LONGUEUR};

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
    let mut width: Vec<f32> = vec![];
    let mut height: Vec<f32> = vec![];
    (width,height)=get_screen(players);


    let mut Active: Vec<ball::Ball> = vec::Vec::new();//un autre thread les genere periodiquement avec direcion random
    let mut off:Vec<ball::Ball> = vec::Vec::new();

    let mut internal_timer = time::Instant::now();

    let mut last_modifier_gen = time::Instant::now();

    let mut rackets:Vec<racket::Racket> = vec::Vec::new();
    for p in players.iter() {
        rackets.push(racket::Racket::new(p, 250.));
        }

    loop {
        for r in rackets.iter_mut() {
            //recvei game data s'occupe d'update leurs valeurs
        }
        for index_b in (1..Active.len()){
            if Active[index_b].posY>0.95*LONGUEUR || Active[index_b].posY<0.05*LONGUEUR{
                off.push(Active.swap_remove(index_b));//changer de liste la balle

            }
            Active[index_b].update_status(rackets, internal_timer)
        }

        internal_timer = time::Instant::now();

        for p in players.iter_mut() {
            send_game_data(p, &Active, &rackets);
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
            //data.push((s.get_life() as u8).to_be());
            //life est juste perdu ou non? 
            //ca devrait etre les coordonnées affichées? ou on affiche sur les tels sur la pos du doigt
            // cad mentir au joueur, deplacer raacket elors que le serveur a pas encore recu

            data.append(&mut r.rect(p));

        }
    }

    //pour l'instatn, on envoie toutes les balles actives, pas celles qui sont visibles du joueur
    data.push((balls.len() as u8).to_be());

    for b in balls.iter() {
        //CONTRAIREMENT A MAZE , on envoie pas la vitesse de la balle , les telephones ne vont pas devine ma pos des balles

        data.append(&mut b.circle(p));
    }
    p.send(&data)
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
                s.speed.x = f32::from_be_bytes(bb);
                bb.copy_from_slice(&buffer[4..8]);
                s.speed.y = f32::from_be_bytes(bb);


            }
        }
//////////////////////////
    }
}
