mod test1;

trait DiskInterface {
    fn boot(&self);
}
struct Disk {}
impl DiskInterface for Disk {
    fn boot(&self) {
        println!("booting disk");
    }
}

trait WebsocketInterface {}
struct Websocket {}
impl WebsocketInterface for Websocket {}

trait BuilderInterface {}
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

fn test1() {
    let client = Client {
        disk: Box::new(Disk {}),
        socket: Box::new(Websocket {}),
        builder: Vec::<Builder>::new(),
    };
    let client = Client2 {
        disk: Disk {},
        socket: Websocket {},
        builder: Builder {},
    };
}

fn main() {
    println!("OK");
}
