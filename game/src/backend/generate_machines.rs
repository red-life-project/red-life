use crate::backend::gamestate::GameState;
use crate::draw;
use crate::machines::machine::Maschine;

///DIESE DATEI IST ZUM TESTEN VON SANDER

impl GameState{
    pub fn create_machien(&mut self){

        let newMS= Maschine::test_maschine(self);
        self.areas.push(Box::new(newMS));
    }
    pub fn new_test_machines(){
        let var:Maschine = Maschine::test_maschine();
        draw!()
    }



}