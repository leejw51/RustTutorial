use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    let mut movie_reviews = BTreeMap::new();

    // review some movies.
    movie_reviews.insert("Office Space", "Deals with real issues in the workplace.");
    {
        let (first_key, first_value) = movie_reviews.iter().next().unwrap();
        println!("{} = {}", first_key, first_value);
    }
    movie_reviews.remove("Office Space");

    movie_reviews.insert("Pulp Fiction", "Masterpiece.");
    movie_reviews.insert("The Godfather", "Very enjoyable.");
    movie_reviews.insert("The Blues Brothers", "Eye lyked it a lot.");
    let mut j = movie_reviews.iter_mut();
    for i in 0..10 {
        if i == 5 {
            //  movie_reviews.insert("The Blues Brothers", "Eye lyked it a lot.");
        }
        {
            match j.next() {
                Some((first_key, first_value)) => {
                    println!("{} = {}", first_key, first_value);
                }
                None => {
                    j = movie_reviews.iter_mut();
                }
            }
        }
    }
    /* for (movie, review) in &movie_reviews {
        println!("{} = {}", movie, review);
    }*/
}
