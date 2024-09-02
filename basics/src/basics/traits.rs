///
/// trait:
///  - set of methods
///  - can contain abstract methods that do not have an implementation
///  - can contain defailt methods that have and implementation
///  - kind of like Python abstract methods
/// 
/// Anything can implement a trait. Implementer will have to provide
/// implementations for the abstract methods.
/// 
/// Implementer can optionally override default methods.
/// 
/// trait-bound
/// 
/// 
/// In this example, we creae a Stack and a Basket. Both are generic over different
/// data structures that can be used to hold data. Additionally, both will implement the same trait. 
/// By doing so, we can create functions that will be able to work on both Stack as well as Basket.
/// 
/// Below you will find:
/// - simple basket
/// - generic basket
/// - traits

#[allow(dead_code)]

pub fn traits_example(){
    println!("traits!");
    let simple_basket = SimpleBasket::new("hi there".to_string());
    let generic_basket1 = GenericBasket::new("hi there".to_string());
    let generic_basket2 = GenericBasket::new(2);
    let generic_basket3 = GenericBasket::new(true);
    let generic_stack_1 = GenericStack::new(vec![1,2,3]);

    let mut generic_trait_basket = GenericBasketWithTrait::new("hi there".to_string());
    let mut generic_trait_stack = GenericStackWithTrait::new(vec!["hi there".to_string(), "goodby".to_string()]);
    add_string(&mut generic_trait_basket, "yo".to_string());
    add_string(&mut generic_trait_stack, "yo".to_string());
}

fn add_string<T: Container<String>>(c: &mut T, s:String){
    c.put(s);
}

pub struct SimpleBasket{
    item: Option<String>,
}

impl SimpleBasket{
    fn new(item:String)->Self{
        SimpleBasket { item : Some(item)}
    }
    // take: takes the value out of the option, leaving a None in its place. If
    // the Option already is a None, it will return a None.    
    fn get(&mut self)->Option<String>{
        self.item.take()
    }

    fn put(&mut self, item:String){
        self.item = Some(item);
    }

    fn is_empty(&self)->bool{
        self.item.is_none()
    }
}
// GenericBasket


pub struct GenericBasket<T>{
    item: Option<T>,
}

impl <T>GenericBasket<T>{
    fn new(item:T)->Self{
        GenericBasket { item : Some(item)}
    }
    // take: takes the value out of the option, leaving a None in its place. If
    // the Option already is a None, it will return a None.    
    fn get(&mut self)->Option<T>{
        self.item.take()
    }

    fn put(&mut self, item:T){
        self.item = Some(item);
    }

    fn is_empty(&self)->bool{
        self.item.is_none()
    }
}


// GenericStack
pub struct GenericStack<T>{
    items: Vec<T>,
}

impl <T>GenericStack<T>{
    fn new(items:Vec<T>)->Self{
        GenericStack { items :items}
    }
    // take: takes the value out of the option, leaving a None in its place. If
    // the Option already is a None, it will return a None.    
    fn get(&mut self)->Option<T>{
        self.items.pop()
    }

    fn put(&mut self, item:T){
        self.items.push(item);
    }

    fn is_empty(&self)->bool{
        if self.items.len() == 0{
            return true
        } else{
            return false
        }
    }
}

// Trait
pub trait Container<T> {
    fn get(&mut self)->Option<T>;        
    
    fn put(&mut self, item:T);

    fn is_empty(&self)->bool;
}


pub struct GenericBasketWithTrait<T>{
    item: Option<T>,
}

impl<T> GenericBasketWithTrait<T>{
    fn new(item:T)->GenericBasketWithTrait<T>{
        return GenericBasketWithTrait{item : Some(item)}
    }
}
impl<T> Container<T> for GenericBasketWithTrait<T>{
    fn get(&mut self)->Option<T>{
        self.item.take()
    }

    fn put(&mut self, item:T){
        self.item = Some(item);
    }

    fn is_empty(&self)->bool{
        self.item.is_none()
    }
}

pub struct GenericStackWithTrait<T>{
    items: Vec<T>,
}

impl<T> GenericStackWithTrait<T>{
    fn new(items: Vec<T>)->Self{
        GenericStackWithTrait{items:items}
    }
}

impl<T> Container<T> for GenericStackWithTrait<T>{
    fn get(&mut self)->Option<T>{
        self.items.pop()
    }

    fn put(&mut self, item:T){
        self.items.push(item);
    }

    fn is_empty(&self)->bool{
        if self.items.len() == 0{
            return true
        } else{
            return false
        }
    }    
}