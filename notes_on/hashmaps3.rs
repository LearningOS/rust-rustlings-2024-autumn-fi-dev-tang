/*
尝试使用哈希表的方式填写数据
*/
use std::collections::HashMap;

struct Team{
    goals_scored: u8,
    goals_concerned: u8,
}

fn build_scores_table(results: String)-> HashMap<String, Team>{
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines(){
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();   
        /*
        parse() 方法转换指定的类型，返回一个 Result 类型
        unwrap() 解析 Result 类型的成功值,Ok(value)则返回 value, 如果是 Err(error), 直接触发 panic
        */
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();

        scores.insert(team_1_name, Team{goals_scored: team_1_score, goals_concerned: team_2_score});
        scores.insert(team_2_name, Team{goals_scored: team_2_score, goals_concerned: team_1_score});
    }
    scores
}

fn get_results() -> String {
    let results = "".to_string()
    + "England,France,4,2\n"
    + "France,Italy,3,1\n"
    + "Poland,Spain,2,0\n"
    + "Germany,England,2,1\n";
    results
}

fn build_scores(){
    let scores = build_scores_table(get_results());

    let mut keys: Vec<&String> = scores.keys().collect();
    keys.sort();
    assert_eq!(
        keys,
        vec!["England", "France", "Germany", "Italy","Poland", "Spain"]
    );
}

fn main(){
    get_results();
    build_scores();
}