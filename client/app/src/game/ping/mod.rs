use crate::network;
use std::convert::TryInto;
use std::ffi::{c_float, c_int};

use c_char;

use raylib::{
    ClearBackground, Color, DrawCircle, DrawFPS, DrawRectanglePro, DrawSplineSegmentBezierCubic,
    DrawSplineSegmentLinear, DrawText, GetScreenHeight, GetScreenWidth, GetSplinePointBezierCubic,
    GetSplinePointLinear, IsMouseButtonDown, MouseButton_MOUSE_BUTTON_LEFT, Rectangle,
    SetTargetFPS, Vector2, WindowShouldClose,
};

use raylib::{draw, raylib_str};

///////CONSTANTES POUR AFFICHAGE
const RADIUS:c_int=20;
const R_WIDTH:c_int=100;
const R_HEIGTH:c_int=20;

const UNDERSPACE:c_int=200;

struct Racket{
posX:c_int,
posY:c_int,
rectangle:Rectangle,
color:Color,
width:c_int,
heigth:c_int,
}

impl Racket{
    fn new() -> Racket {
        
        Racket{
        posX:UNDERSPACE,
        posY:GetScreenHeight()/2,//renvoie quotient de la div euc
            

        //on le fait STUPIDEMENT  d'abord : le server envoie la pos et on recoit la pos
        //AU LIEU  de faire confiance au doigt de l'utilisateur et afficher là où il est posé

        rectangle: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 200.0,
            },
        color: Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            },
        width:R_WIDTH,
        heigth:R_HEIGTH,
        }
    }

    unsafe fn draw(&self) {
        DrawRectanglePro(
            Rectangle {
                x: self.posX,
                y: self.posY,
                width: self.rectangle.width,
                height: self.rectangle.height,
            },
            Vector2 {
                x: self.rectangle.width / 2.0,
                y: self.rectangle.height / 2.0,
            },
            0. as f32,
            self.color,
        );


}
}

struct Ball{
    posX:c_int,
    posY:c_int,
    color:Color,
    radius:c_int,
    }
    
    impl Ball{
        fn new(initX:c_int,initY:c_int) -> Ball {
            
            Ball{
            posX:initX,
            posY:initY,
                
    
            //on le fait STUPIDEMENT  d'abord : le server envoie la pos et on recoit la pos
            //AU LIEU  de faire confiance au doigt de l'utilisateur et afficher là où il est posé
            color: Color {
                    r: 255,
                    g: 255,
                    b: 0,
                    a: 255,
                },
            radius:RADIUS,
            }
        }
    
        unsafe fn draw(&self) {
            DrawCircle(
                posX,
                posY,
                radius as f32,
                self.color,
            );
    
    
    }
}

pub unsafe fn main_game(network: &mut network::Network) {
    let (width, height) = (GetScreenWidth(), GetScreenHeight());

    let mut buffer = [0_u8; network::packet::MAX_DATA_SIZE];
    while network.recv(&mut buffer) == 0 {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    //L'initialisation des variables est "faite de force"
    //pour l'instant, la taille du terrain de dépend pas du nombre de joueurs

    let mut ID:c_int=0;
    let mut actives: Vec<Ball> = vec::Vec::new();
    let mut visibles:Vec<Ball> = vec::Vec::new();
    let mut racket:Racket=Racket::new();

    let mut update_pos = raylib::Vector2 { x: 0., y: 0. };

    let mut internal_timer = time::Instant::now();

    loop {
        let mut buffer = [0_u8; packet::MAX_DATA_SIZE];
        let mut anex = [0_u8; packet::MAX_DATA_SIZE];
        let n1 = network.recv(&mut anex);
        let mut n = 0;
        loop {
            buffer.copy_from_slice(&anex);
            n = network.recv(&mut anex);
            if n == 0 {
                break;
            }
        }
        if n1 > 0 {
            //we unpack the info from server here

            //en focntion de l'ID recu, on connait quellle position on a sur le terrain
            // et donc quelles balles afficher

            //on modiifie ces variables et on peut renvoyer l'ID du joueur pour verifier identité 
            (ID,actives) = unpack_game_data(&buffer,&racket);

            visibles.clear();//le telephone recoit TOUTES les balles et affiche les visible
            //ou le server envoie à chacun lequelles sont visibles

            //pour l'instant/debug , affiche TOUTES les balles meme en dehors de l'ecran 
            /*for b in actives{
                if (){
                    visibles.append(b)
                }
            }*/

        } else {
            racket.posX=update_pos.y;//le tel actualise la pos en suivant le doigt
            //avec le bloc au dessus, la raquette va "blink" à la position à laquelle le servver croit qu'elle est
            }

            // update bullet status
            //c'est le server qui s'occupe de calculer la pos et collision des balles??
                // pour l'instant pas de :
                //"-Mais sur mon écran je t'ai touché??
                //- Eh ben sur MON ecran tu m'as raté de 12 pixels sur la gauche" 
                /* 
            let mut i = 0;
            while i < bullets.len() {
                
            }
            */
        }
        internal_timer = time::Instant::now();

        if raylib::IsMouseButtonDown(raylib::MouseButton_MOUSE_BUTTON_LEFT.try_into().unwrap()) {
            update_pos = raylib::GetMousePosition();
        }

        //send update_pos en tant que la coord de notre racket. Donc avec notre ID player associé??
        // ou c'est fait automatiquement?
        send(network, racket.posX);




        raylib::draw!({
            raylib::ClearBackground(raylib::Color {
                r: 65,
                g: 65,
                b: 65,
                a: 255,
            });

            //pas de textures dans mon ping

            for b in actives.iter() {
                b.draw();
            }
            racket.draw();



        });
    }



//////////////////////////////////////////////
///
///
/// Unpack game data
///
///
//////////////////////////////////////////////

fn unpack_game_data(
    _data: &[u8],
    rack: Racket
) -> (c_int, Vec<Ball>) {
    let id = u8::from_be(_data[0]);
    let mut buffer = [0_u8; 4];
    data = &_data[1..];//pourquoi ca passe de _data à data?

    //la racket, on veut juste le pos/posX
    buffer.copy_from_slice(&data[0..4]);
    let pos_x = f32::from_be_bytes(buffer);
    buffer.copy_from_slice(&data[4..8]);
    let pos_y = f32::from_be_bytes(buffer);

    rack.posY=pos_y;

    //on envoie aussi sizeX et sizeY pour voir les hitboxes au cas où

    data=&data[16..];

    let mut n = u8::from_be(data[0]) as usize;//le nombre de balles

    data = &data[1..];
    let mut balls = vec::Vec::new();

    for i in 0..n {

        buffer.copy_from_slice(&data[i * 12..i * 12 + 4]);
        let pos_x = f32::from_be_bytes(buffer);
        buffer.copy_from_slice(&data[i * 12 + 4..i * 12 + 8]);
        let pos_y = f32::from_be_bytes(buffer);
        buffer.copy_from_slice(&data[i * 12 + 8..i * 12 + 12]);
        let rayon = f32::from_be_bytes(buffer);

        balls.push(Ball::new(pos_x,pos_y));

        
    }

    (id, balls)
}

//////////////////////////////////////////////
///
///
/// Sender
///
///
//////////////////////////////////////////////

fn send(network: &mut network::Network,pos:c_int) {
    let mut data = [0_u8; 4];

    let mut buffer = pos.to_be_bytes();
    data[..4].copy_from_slice(&buffer);

    network.send(&data).unwrap();
}
//on envoie sans s'embeter les coordonnees de la raquette
