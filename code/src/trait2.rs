trait HashComputer
{
     fn compute(&self) ->i64;
}

trait HashUser
{
    fn use_hash(&self);
}
struct QuantumHashComputer {

}

impl HashComputer for QuantumHashComputer {
     fn compute(&self)->i64 {
        100 as i64
    }
}

struct Network<'a, T> where T: HashComputer {
    hash: &'a T  
}

impl<'a,T> HashUser for Network<'a,T> where T: HashComputer {
    fn use_hash(&self) {
        println!("network hash={}", self.hash.compute());
    }
}



struct Wallet<'a,T> where T: HashComputer {
    hash: &'a T
}

impl<'a,T> HashUser for Wallet<'a,T> where T: HashComputer {
    fn use_hash(&self) {
        println!("wallet hash={}", self.hash.compute());
    }
}

fn main () {
    let hash = QuantumHashComputer{};
    let network= Network{hash:&hash};
    let wallet = Wallet{hash:&hash};
    wallet.use_hash();
    network.use_hash();
}