// ISP
trait Fly {
    fn fly(&self);
}

trait Work {
    fn work(&self);
}

trait Swim {
    fn swim(&self);
}

struct Bird {
    name: String,
}

struct Fish {
    name: String,
}

impl Fly for Bird {
    fn fly(&self) {
        println!("{} fly!", self.name);
    }
}

impl Swim for Fish {
    fn swim(&self) {
        println!("{} swim", self.name);
    }
}

pub fn run() {
    // それぞれの構造体は必要な機能のみに依存する
    // BirdとFishが一緒になったようなインターフェースを作ってはならない
    let bird = Bird {
        name: "鴎".to_string(),
    };

    let fish = Fish {
        name: "鰤".to_string(),
    };

    bird.fly();
    fish.swim();
}
