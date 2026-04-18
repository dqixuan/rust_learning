
// define shared behavior with traits

// difine a trait
pub trait Summary {
    fn summarize() {
        //  there can have a default use here.
    }
}

// implement the Summary trait
pub struct NewUse {

}

impl Summary for NewUse {
    fn summarize() {
        
    }
}