mod permute;

fn main() {
    let vecs = permute(vec![1,2,3]);
    for vec in vecs {
        for e in vec {
            print!("{} ", e);
        }
        print!("\n");
    }
}