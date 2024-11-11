use std::f32;

fn main() {
    ufc_grades();
    //find_biggest_smallest();
    //println!("Hello, world!");
    //tuple_exercise();
}

fn tuple_exercise() {
    let p1 : (i32,i32,i32) = (4,5,6);
    let p2 : (i32,i32,i32) = (7,8,9);
    let dist : i32 = (p1.0 - p2.0)*(p1.0 - p2.0) +
                     (p1.1 - p2.1)*(p1.1 - p2.1) +
                     (p1.2 - p2.2)*(p1.2 - p2.2);
    let dist : f32 = dist as f32;
    println!("The distance is {:.1}", dist.sqrt());
}

fn find_biggest_smallest() {
    
    let array = [45,10,-30,100,23,15];
    let mut biggest = array[0];
    let mut smallest = array[0];
    
    for elem in array.iter() {
        if *elem > biggest {
            biggest = *elem;
        }
        if *elem < smallest {
            smallest = *elem;
        }
    }

    println!("The biggest is: {biggest}");
    println!("The smallest is: {smallest}");
}

fn ufc_grades() {

    let np1 = 8.4;
    let np2 = 5.3;
    let media = (np1 + np2) / 2.0;
    println!("Media: {}", media);
    if media > 7 { println!("Aprovado conceito !"); }
    else if media < 4 { println!("Reorovado!"); }
    else {
        println!("Prova final!");
    }

}