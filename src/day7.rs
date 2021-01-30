use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;
use daggy::NodeIndex;
use daggy::{Dag, Walker};


fn count_parents<'a>(dag: &'a Dag::<u32, u32, u32>, walker: daggy::Parents<u32, u32, u32>, all_parents: &'a mut HashSet<NodeIndex>) {
    for (_e, n) in walker.iter(dag) {
        count_parents(dag, dag.parents(n), all_parents);
        all_parents.insert(n);
    }
}

fn count_children(dag: &Dag::<u32, u32, u32>, walker: daggy::Children<u32, u32, u32>) -> u32 {
    let mut sub_bags = 0;
    for (e, n) in walker.iter(dag) {
        let edge_weight = dag.edge_weight(e).unwrap();
        let child_bags = count_children(dag, dag.children(n));
        sub_bags += edge_weight*child_bags+edge_weight;
    }
    sub_bags
}

fn add_bag(dag: &mut Dag::<u32, u32, u32>, all_bags: &mut HashMap<String, NodeIndex>, bag_key: &str) {
    if !all_bags.contains_key(bag_key) {
        all_bags.insert(
            bag_key.to_string(),
            dag.add_node(1),
        );
    }
}

pub fn day7() {
	let file = File::open("day7_input.txt").expect("file not found!");
    let reader = BufReader::new(file);

    let mut dag = Dag::<u32, u32, u32>::new();
    let mut all_bags: HashMap<String, NodeIndex> = HashMap::new();
    let mut edges: Vec<(NodeIndex, NodeIndex, u32)> = Vec::new();

	for line in reader.lines() {
        let text = &line.unwrap().replace("bags", "bag").replace(".", "");
        // println!("text: {:?}", text);
        let mut kv_pair = text.split(" contain ");
        let (parent_key, sub_bags) = (kv_pair.next().unwrap(), kv_pair.next().unwrap());
        add_bag(&mut dag, &mut all_bags, parent_key);
        for i in sub_bags.split(", ") {
            if i != "no other bag" {
                let mut sub_bag = i.splitn(2, ' ');
                let (weight, child_key) = (sub_bag.next().unwrap(), sub_bag.next().unwrap());
                add_bag(&mut dag, &mut all_bags, child_key);
                edges.push((all_bags[parent_key], all_bags[child_key], weight.parse::<u32>().unwrap()))
            }
        }
    }
    dag.add_edges(edges.into_iter()).unwrap();
    
    let mut all_parents: HashSet<NodeIndex> = HashSet::new();
    count_parents(&dag, dag.parents(all_bags["shiny gold bag"]), &mut all_parents);
    println!("possible parents: {:?}", all_parents.len());

    let number_nested =  count_children(&dag, dag.children(all_bags["shiny gold bag"]));
    println!("nested bags: {:?}", number_nested);
}
