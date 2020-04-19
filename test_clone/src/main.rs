trait DiskInterface {
    fn boot(&self);
}
#[derive(Default)]
struct Disk {}
impl DiskInterface for Disk {
    fn boot(&self) {
        println!("booting disk");
    }
}

trait WebsocketInterface {}

#[derive(Default)]
struct Websocket {}
impl WebsocketInterface for Websocket {}

trait BuilderInterface {}
#[derive(Default)]
struct Builder {}
impl BuilderInterface for Builder {}

#[derive(Default)]
struct Client<D, W, B>
where
    D: DiskInterface,
    W: WebsocketInterface,
    B: BuilderInterface,
{
    pub disk: Box<D>,
    pub socket: Box<W>,
    pub builder: Vec<B>,
}

#[derive(Default)]
struct Client2<D, W, B>
where
    D: DiskInterface,
    W: WebsocketInterface,
    B: BuilderInterface,
{
    pub disk: D,
    pub socket: W,
    pub builder: B,
    pub client: Client<D, W, B>,
}

type MyClient2 = Client2<Disk, Websocket, Builder>;

#[derive(Default)]
struct Program {
    client2: MyClient2,
}

impl Program {
    pub fn initialize(&mut self) {
        let client = Client {
            disk: Box::new(Disk {}),
            socket: Box::new(Websocket {}),
            builder: Vec::<Builder>::new(),
        };
        let client2 = Client2 {
            disk: Disk {},
            socket: Websocket {},
            builder: Builder {},
            client,
        };
        self.client2 = client2;
    }
    // add code here
}

pub fn main() {
    println!("OK");
    let mut p = Program::default();
    p.initialize();
}
