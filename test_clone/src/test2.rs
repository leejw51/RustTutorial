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
}

type MyClient2 = Client2<Disk, Websocket, Builder>;

#[allow(dead_code)]
fn test1() {
    let _client = Client {
        disk: Box::new(Disk {}),
        socket: Box::new(Websocket {}),
        builder: Vec::<Builder>::new(),
    };
    let _client = Client2 {
        disk: Disk {},
        socket: Websocket {},
        builder: Builder {},
    };
}

#[derive(Default)]
struct Program {
    _client: MyClient2,
}

#[allow(dead_code)]
pub fn main() {
    println!("OK");
    let _p = Program::default();
}
