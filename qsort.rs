
fn qsort(xs: &Vec<i32>) -> Vec<i32>
{
    if xs.len()<=0 {
        return vec![];
    }
    let n= xs.len();
    let first= xs[0];
    // from index 1, to the end
    let mut remain: Vec<i32>= xs[1..].to_vec();
    let mut smaller:Vec<i32>= vec![];
    let mut same:Vec<i32>= vec![];
    let mut bigger:Vec<i32> = vec![];

    for x in remain {
        if x< first {
            smaller.push(x);
        }
        else if x== first {
            same.push(x);
        }
        else if x> first {
            bigger.push(x);
        }
    }    

    let mut newone:Vec<i32>= vec![];
    let mut newsmaller= qsort(&smaller);
    let mut newbigger = qsort(&bigger);
    newone.append(&mut newsmaller);
    newone.append(&mut same);
    newone.push(first);
    newone.append(&mut newbigger);
    return newone;
}
fn main() 
{
    let mut xs:Vec<i32>= vec! [60,10,50, 15,5];
  
    let mut ys:Vec<i32>= vec! [11,202,203];
    xs.append(&mut ys);
    let b= qsort(&xs);
    println!("final={:?}", b);

}