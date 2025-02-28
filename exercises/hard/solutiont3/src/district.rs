use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

type CityPair = (String, Vec<String>);

#[derive(Debug)]
struct District {
    cities: Vec<CityPair>,
}


impl District {
    fn new() -> Self {
        District {
            cities: Vec::new(),
        }
    }

    fn add_city(&mut self, city: String, neighbors: Vec<String>) {
        self.cities.push((city, neighbors));
    }
}

fn parse_json(content: &str) -> HashMap<String, District> {
    let mut districts = HashMap::new();
    let mut current_district = String::new();
    let mut in_district = false;
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        
        if line.contains(":") {
            if line.ends_with("{") {
                // 处理district编号
                if let Some(district_num) = line.split('"').nth(1) {
                    current_district = district_num.to_string();
                    districts.entry(current_district.clone())
                        .or_insert_with(District::new);
                    in_district = true;
                }
            } else if in_district && !line.ends_with("{") {
                // 处理城市和邻居
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() == 2 {
                    // 处理城市名
                    let city = parts[0].trim_matches(|c| c == '"' || c == ' ').to_string();
                    
                    // 处理邻居列表
                    let neighbors_str = parts[1].trim();
                    if neighbors_str.starts_with('[') {
                        // 过滤掉空字符串和清理字符串两端的特殊字符
                        let neighbors: Vec<String> = neighbors_str
                            .trim_matches(|c| c == '[' || c == ']' || c == ',')
                            .split(',')
                            .map(|s| s.trim_matches(|c| c == '"' || c == ' ' || c == '[' || c == ']'))
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect();
                        
                        if let Some(district) = districts.get_mut(&current_district) {
                            if !neighbors.is_empty() {
                                district.add_city(city, neighbors);
                            }
                        }
                    }
                }
            }
        }
    }
    
    districts
}

struct Graph {
    adj_list: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        self.adj_list.entry(from.to_string())
            .or_insert_with(HashSet::new)
            .insert(to.to_string());
        self.adj_list.entry(to.to_string())
            .or_insert_with(HashSet::new)
            .insert(from.to_string());
    }

    fn count_connected_components(&self) -> i32 {
        let mut visited = HashSet::new();
        let mut count = 0;
        let mut stack = VecDeque::new();

        // 处理所有节点
        for node in self.adj_list.keys() {
            if !visited.contains(node) {
                count += 1;
                stack.push_back(node.clone());
                visited.insert(node.clone());

                // 使用栈进行DFS
                while let Some(current) = stack.pop_back() {
                    if let Some(neighbors) = self.adj_list.get(&current) {
                        for neighbor in neighbors {
                            if !visited.contains(neighbor) {
                                stack.push_back(neighbor.clone());
                                visited.insert(neighbor.clone());
                            }
                        }
                    }
                }
            }
        }
        count
    }
}

pub fn count_provinces() -> String {
    let file_content = fs::read_to_string("district.json")
        .expect("Failed to read district.json");
    
    let districts = parse_json(&file_content);
    // println!("{:#?}", districts.get("5"));
    let mut result = Vec::new();
    
    for i in 1..=5 {
        let mut graph = Graph::new();
        
        if let Some(district) = districts.get(&i.to_string()) {
            for (city, neighbors) in &district.cities {
                for neighbor in neighbors {
                    graph.add_edge(city, neighbor);
                }
            }

            result.push(graph.count_connected_components());
        } else {
            result.push(0);
        }
    }

    result.iter()
        .map(|&x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
