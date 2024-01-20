/*use crate::game::bezier::*;
use crate::game::vehicle::*;
use plotters::prelude::*;
use tqdm::tqdm;
*/

use std::time;
use std::vec;

//MODULES
use crate::network::player;

mod ball;
mod racket;





//Constantes DIMENSIONNELLES
const LONGUEUR: f64 = 2000.;//virtuelles
const LARGEUR: f64 = 500.;//virtuelle

const SPEED_EXCESS: f64 = 0.3;
const FRICTION: f64 = 0.1;
const DT: f64 = 1. / 60. * 1.; // * 0.01 mieux

/////////////////////////
/// 
/// Le terrain du jeu
/// 
/// 
////////////////////////

//Composé d'un rrectangle qui est de taille MAX(p.physycal_width),MAX(p.physycal_height)
//Les coordonnées de tous les objets sont dans le rectangle PUIS l'affichage est customisé par telephone

//Une bande bleue/rouge en haut et en bas pour indiquer direction sur les tels

//Bande [couleur] hachurée pour le buts

pub struct Game {
    players: Vec<Racket>,
    balls:   Vec<Ball>,
}

impl Game {
    /// Create a new game structure. If the map parameter is an empty list,
    /// the map is randomly created.
    pub fn new(
        mut map: Vec<Bezier>,
        n_cars: usize,
        dimensions: &Vec<(f64, f64)>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut cars = Vec::new();
        for i in 0..n_cars {
            cars.push(Vehicle::new(0, i));
        }
        if map.is_empty() {
            let io_map = Game::get_io_map(dimensions)?;
            map = Bezier::random_map(dimensions, io_map);
        }
        // assert!(
        //     map[0].get_points().0 == map[map.len() - 1].get_points().3,
        //     "The given circuit is not closed. Please make sure the first and last point coincide."
        // );
        Ok(Game { map, cars })
    }

    /// Generate the points at a third of the minimal height of two consecutive phones to build the Bezier curves. The circuit is build anticlockwise.

    #[allow(unused)]
    fn get_map(&self) -> Vec<Bezier> {
        self.map.clone()
    }



}

//un vecteur de vecteur pour permettre de 
//convertir des coord en physique et de celles en virtuel
pub fn gen_map(players: &mut [player::Player]) -> (Vec<f32> ,Vec<f32>) {
    // for now build a map of the width the sum of width and heigh max(height)
    let mut width = 0.;
    let mut height = 0.;

    let mut W: Vec<f32> ;
    let mut H: Vec<f32> ;
    for p in players.iter() {
        width += p.physical_width;
        height = p.physical_height.max(height);
        
        W.push(p.physical_width);
        H.push(p.physical_height);

    }



}


#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = f64::EPSILON * 10.;
    #[test]
    fn test_io_map() {
        // Can generate the points as required
        let phone_size = vec![(1., 1.), (0.3, 0.3), (1., 1.)];
        let io_map = Game::get_io_map(&phone_size).unwrap();
        assert_eq!(io_map.len(), 4 * (phone_size.len() - 1));
        assert!(io_map
            .iter()
            .zip(vec![
                ((0.9, 0.55), (0.9, 0.45)),
                ((0.9, 0.45), (1.1, 0.45)),
                ((1.1, 0.45), (1.2, 0.45)),
                ((1.2, 0.45), (1.4, 0.45)),
                ((1.4, 0.45), (1.4, 0.55)),
                ((1.4, 0.55), (1.2, 0.55)),
                ((1.2, 0.55), (1.1, 0.55)),
                ((1.1, 0.55), (0.9, 0.55)),
            ])
            .all(|(&(f1, f2, _, _), (f3, f4))| (f1.0 - f3.0) < EPSILON
                && (f1.1 - f3.1) < EPSILON
                && (f2.1 - f4.1) < EPSILON
                && (f2.0 - f4.0) < EPSILON));
    }

    #[test]
    fn test_random_map_bezier() {
        let phone_size = vec![(1., 1.), (0.3, 0.3), (1., 1.)];
        let game = Game::new(Vec::new(), 1, &phone_size).unwrap();
        let map = game.get_map();
        assert!(map
            .iter()
            .enumerate()
            .all(|(i, curve)| { curve.get_points().3 == map[(i + 1) % map.len()].get_points().0 }));
    }
    #[test]
    #[should_panic]
    fn test_panic_one_phone() {
        let phone_size = vec![(1., 1.)];
        Game::new(Vec::new(), 1, &phone_size).unwrap();
    }
}
