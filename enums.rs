struct AadharCard{
    name:String,
    gender:Gender,
    date_of_birth:String,
    aadhar_number:String,
    address:String,
}

// enums 
#[derive(Debug)]
enum Gender {
    male,
    female,
}

impl AadharCard{
    fn display_details(&self){
        println!("name : {}",self.name);
        println!("gender : {:?}",self.gender);
        println!("date of birth : {}",self.date_of_birth);
        println!("aadhar number :{}",self.aadhar_number);
        println!("address : {}",self.address)
    }
    
    // matching
    fn is_male(&self)->bool{
        match self.gender{
            Gender::male=>true,
            _=>false,
        }
    }
    fn validate_aadhar_number(&self)->bool{
        true
    }
}

fn main() {
    println!("Hello, world!");
    let card= AadharCard{
        name:"prxthxmesh".to_string(),
        gender:Gender::male,
        date_of_birth:"11/08/2002".to_string(),
        aadhar_number:"973364772972933".to_string(),
        address:"navi-mumbai".to_string(),
    };
    card.display_details();

    if card.is_male(){
        println!("card belongs to male cardholder")
    }
    else{
        println!("card does not belongs to male cardholder")
    }
}