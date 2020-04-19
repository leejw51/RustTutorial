mod test1;
mod test2;

trait DiskInterface: Sync + Sync + Clone {
    fn boot(&self);
}
#[derive(Default, Clone)]
struct Disk {}
impl DiskInterface for Disk {
    fn boot(&self) {
        println!("booting disk");
    }
}

trait WebsocketInterface: Sync + Send + Clone {}

#[derive(Default, Clone)]
struct Websocket {}
impl WebsocketInterface for Websocket {}

trait BuilderInterface: Sync + Send + Clone {}
#[derive(Default, Clone)]
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
    pub builder: Box<B>,
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
            builder: Box::new(Builder {}),
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

#[derive(Default)]
struct Program2<D, W, B>
where
    D: DiskInterface,
    W: WebsocketInterface,
    B: BuilderInterface,
{
    client2: Client2<D, W, B>,
}

impl<D, W, B> Program2<D, W, B>
where
    D: DiskInterface,
    W: WebsocketInterface,
    B: BuilderInterface,
{
    pub fn initialize(&mut self, disk: D, socket: W, builder: B) {
        let client = Client {
            disk: Box::new(disk.clone()),
            socket: Box::new(socket.clone()),
            builder: Box::new(builder.clone()),
        };
        self.client2 = Client2 {
            disk: disk.clone(),
            socket: socket.clone(),
            builder: builder.clone(),
            client,
        };
    }
    // add code here
}

pub fn main() {
    println!("OK");
    let mut p = Program::default();
    p.initialize();

    let mut p2: Program2<Disk, Websocket, Builder> = Program2::default();
    let _client = Client {
        disk: Box::new(Disk {}),
        socket: Box::new(Websocket {}),
        builder: Box::new(Builder {}),
    };
    p2.initialize(Disk {}, Websocket {}, Builder {});
}
