fn main() {
    println!("Hello, world!");
    #[derive(Debug)]
    enum Gender {
        Male(u8),
        Male(u8),
    }
    
    let sudham = Gender :: Male(16);
    let prathamesh = Gender :: Male(15);
    println!("hello, {:?}",sudham);
    println!("hello, {:?}",prathamesh);

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let prxthxmesh= Message::Move{x:1,y:2};
    println!("hello {:?}", prxthxmesh);

    let m = Message::Write(String::from("hello"));
    println!("{:?} india",m);
    m.call();

    impl Message{
        fn call(&self) {
            if let Message::Move{x,y}=self{
               println!("x : {}, y : {}",x,y); 
            }
        }
    }
    let xyz=Message::Move{x:11,y:57};
    xyz.call();


    // new move
    struct Move1{x:i32,y:i32};
    impl Move1{
        fn call(&self){
            println!("hello, {:?}",self.x);
        }
    }

    let new_move=Move1{
            x:10,
            y:20,
    };
    new_move.call();
}