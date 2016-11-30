fn main() {

   loop{
       check_if_safe(format!("Safe {}", "Borrows"));
   }
}


fn check_if_safe(s_borrow: String){

     println!("Pure, {}!", s_borrow); 
}
