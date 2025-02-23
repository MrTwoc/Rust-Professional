/*
 * 本模块主要用于处理包含城市邻接关系的 JSON 文件，计算每个批次中城市的连通分量（省份数）。
 * 整体流程包括读取 JSON 文件、解析数据、构建邻接表，最后计算连通分量。
 */

// 借助了强大的AI，帮我学习了这种算法。。
// 所以用 AI 帮我生成了这些详细的注释，方便我以后复习 
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use serde_json;

/*
 * 读取指定路径的 JSON 文件内容。
 * 
 * 实现原理：
 * 利用 `std::fs::read_to_string` 函数读取文件内容，若读取失败则使用 `expect` 方法抛出错误。
 * 
 * 参数：
 * - file_path: 待读取的 JSON 文件的路径。
 * 
 * 返回值：
 * 文件内容的字符串表示。
 */
fn read_json_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read file")
}

/*
 * 解析 JSON 格式的字符串为 `HashMap<String, HashMap<String, Vec<String>>>` 类型的数据。
 * 
 * 实现原理：
 * 使用 `serde_json::from_str` 函数将 JSON 字符串解析为 Rust 数据结构，若解析失败则使用 `expect` 方法抛出错误。
 * 
 * 参数：
 * - json_content: 待解析的 JSON 字符串。
 * 
 * 返回值：
 * 解析后的 Rust 数据结构。
 */
fn parse_json_data(json_content: &str) -> HashMap<String, HashMap<String, Vec<String>>> {
    serde_json::from_str(json_content).expect("Failed to parse JSON")
}

/*
 * 过滤掉城市邻居列表中的自环（即移除与城市自身相同的邻居）。
 * 
 * 实现原理：
 * 遍历邻居列表，使用 `filter` 方法过滤掉与城市名称相同的邻居，然后使用 `map` 方法将剩余的邻居转换为 `&str` 类型，最后收集到 `Vec` 中。
 * 
 * 参数：
 * - city_name: 城市名称。
 * - neighbor_list: 该城市的邻居列表。
 * 
 * 返回值：
 * 过滤后的邻居列表，元素类型为 `&str`。
 */
fn filter_self_neighbors<'a>(city_name: &'a str, neighbor_list: &'a [String]) -> Vec<&'a str> {
    neighbor_list.iter().filter(|&n| n != city_name).map(|n| n.as_str()).collect()
}

/*
 * 根据批次数据构建城市的邻接表。
 * 
 * 实现原理：
 * 1. 首先为所有城市（包括键和邻居）创建邻接表的条目。
 * 2. 然后为每个城市添加其有效的邻居，并确保边是双向的。
 * 
 * 参数：
 * - batch_data: 批次数据，包含城市及其邻居信息。
 * 
 * 返回值：
 * 构建好的邻接表，键为城市名称，值为该城市的邻居集合。
 */
fn build_adjacency_list(batch_data: &HashMap<String, Vec<String>>) -> HashMap<String, HashSet<String>> {
    let mut adjacency_list: HashMap<String, HashSet<String>> = HashMap::new();

    // 为所有城市（键和邻居）创建条目
    for (city_name, neighbor_list) in batch_data {
        let valid_neighbors = filter_self_neighbors(city_name, neighbor_list);
        if !valid_neighbors.is_empty() || batch_data.keys().any(|k| batch_data[k].contains(city_name) && k != city_name) {
            adjacency_list.entry(city_name.clone()).or_insert_with(HashSet::new);
            for neighbor in valid_neighbors {
                adjacency_list.entry(neighbor.to_string()).or_insert_with(HashSet::new);
            }
        }
    }

    // 添加双向边
    for (city_name, neighbor_list) in batch_data {
        let valid_neighbors = filter_self_neighbors(city_name, neighbor_list);
        for neighbor_city in valid_neighbors {
            // 分别处理两个插入操作，避免同时可变借用
            if let Some(city_set) = adjacency_list.get_mut(city_name) {
                city_set.insert(neighbor_city.to_string());
            }
            if let Some(neighbor_set) = adjacency_list.get_mut(neighbor_city) {
                neighbor_set.insert(city_name.clone());
            }
        }
    }

    adjacency_list
}

/*
 * 计算邻接表中城市的连通分量（省份数）。
 * 
 * 实现原理：
 * 使用广度优先搜索（BFS）算法遍历邻接表。从一个未访问的城市开始，将其标记为已访问，并将其邻居加入队列，不断扩展直到队列为空。每开始一次新的 BFS 遍历，省份数加 1。
 * 
 * 参数：
 * - adjacency_list: 城市的邻接表。
 * 
 * 返回值：
 * 连通分量的数量，即省份数。
 */
fn count_connected_components(adjacency_list: &HashMap<String, HashSet<String>>) -> u32 {
    let mut visited_cities = HashSet::new();
    let mut province_count = 0;

    for city in adjacency_list.keys() {
        if !visited_cities.contains(city) {
            province_count += 1;
            let mut city_queue = VecDeque::new();
            city_queue.push_back(city);
            visited_cities.insert(city);

            while let Some(current_city) = city_queue.pop_front() {
                for adjacent_city in adjacency_list.get(current_city).unwrap() {
                    if !visited_cities.contains(adjacent_city) {
                        visited_cities.insert(adjacent_city);
                        city_queue.push_back(adjacent_city);
                    }
                }
            }
        }
    }

    province_count
}

/*
 * 主函数，计算每个批次中城市的连通分量（省份数），并将结果以逗号分隔的字符串形式返回。
 * 
 * 实现原理：
 * 1. 读取 JSON 文件内容。
 * 2. 解析 JSON 数据。
 * 3. 对批次名称进行排序。
 * 4. 遍历每个批次，构建邻接表并计算连通分量。
 * 5. 将每个批次的连通分量数量转换为字符串并以逗号连接。
 * 
 * 返回值：
 * 每个批次的连通分量数量以逗号分隔的字符串。
 */
pub fn count_provinces() -> String {
    let file_content = read_json_file("district.json");
    let parsed_data = parse_json_data(&file_content);

    let mut batch_list: Vec<String> = parsed_data.keys().cloned().collect();
    batch_list.sort();

    let mut province_count_list = Vec::new();

    for current_batch in batch_list {
        let current_batch_data = &parsed_data[&current_batch];
        let adjacency_list = build_adjacency_list(current_batch_data);
        let province_count = count_connected_components(&adjacency_list);
        province_count_list.push(province_count.to_string());
    }

    province_count_list.join(",")
}
