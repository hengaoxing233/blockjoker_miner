use std::sync::{Arc, RwLock};
use std::{env, thread};
use std::net::IpAddr;
use std::time::Duration;
use rand::{Rng};
use sha2::{Sha256, Digest};
use hex;
use reqwest;
use num_cpus;
use serde_json::Value;
use std::sync::atomic::{AtomicBool, Ordering};
fn update_session(session: &Arc<RwLock<String>>, response: &reqwest::blocking::Response) {
    if let Some(cookie) = response.headers().get("set-cookie") {
        if let Ok(cookie_str) = cookie.to_str() {
            if cookie_str.starts_with("session=") {
                let mut session_writer = session.write().unwrap();
                *session_writer = cookie_str.to_string();
                // println!("更新session: {}", cookie_str);
            }
        }
    }
}
fn generate_random_ip() -> IpAddr {
    let mut rng = rand::thread_rng();
    IpAddr::V4(std::net::Ipv4Addr::new(
        rng.gen_range(1..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(1..255)
    ))
}
fn create_shared_client() -> Arc<reqwest::blocking::Client> {
    Arc::new(
        reqwest::blocking::Client::builder()
            .http1_title_case_headers()
            .danger_accept_invalid_certs(true)
            .connection_verbose(true)
            .build()
            .unwrap(),
    )
}
fn fetch_records(token: String, session: Arc<RwLock<String>>) {
    let client = create_shared_client();
    loop {
        let current_session = session.read().unwrap().clone();
        let random_num: IpAddr = generate_random_ip();
        let res = client.get("https://test2.blockjoker.org/api/v1/missions/records")
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
            .header("Authorization", format!("Bearer {token}"))
            .header("Referer", "https://test2.blockjoker.org/home")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36 Edg/127.0.0.0")
            .header("Host", "test2.blockjoker.org")
            .header("Origin", "test2.blockjoker.org")
            .header("Connection", "Keep-Alive")
            .header("Cookie",current_session.to_string())
            .header("Client-Ip", &random_num.to_string())
            .header("X-Forwarded-For", &random_num.to_string())
            .header("Remote_Addr", &random_num.to_string())
            .send();
        match res {
            Ok(response) => {
                // update_session(&session, &response);
                if let Ok(json) = response.json::<Value>() {
                    println!("最后一条奖励：{}",json["result"][0]);
                }
            },
            Err(e) => println!("查询最后一条奖励失败: {:?}", e),
        }
        thread::sleep(Duration::from_secs(30));
    }
}
fn fetch_point(token: String, session: Arc<RwLock<String>>) {
    let client = create_shared_client();
    loop {
        let current_session = session.read().unwrap().clone();
        let random_num: IpAddr = generate_random_ip();
        let res = client.get("https://test2.blockjoker.org/api/v1/accounts")
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
            .header("Authorization", format!("Bearer {token}"))
            .header("Referer", "https://test2.blockjoker.org/home")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36 Edg/127.0.0.0")
            .header("Host", "test2.blockjoker.org")
            .header("Origin", "test2.blockjoker.org")
            .header("Connection", "Keep-Alive")
            .header("Cookie",current_session.to_string())
            .header("Client-Ip", &random_num.to_string())
            .header("X-Forwarded-For", &random_num.to_string())
            .header("Remote_Addr", &random_num.to_string())
            .send();
        match res {
            Ok(response) => {
                // update_session(&session, &response);
                if let Ok(json) = response.json::<Value>() {
                    println!("当前积分：{}",json["result"]["point"]);
                }
            },
            Err(e) => println!("查询积分失败: {:?}", e),
        }

        thread::sleep(Duration::from_secs(30));
    }

}
fn fetch_salt(salt: Arc<RwLock<String>>, token: String, session: Arc<RwLock<String>>) {
    let client = create_shared_client();
    loop {
        let current_session = session.read().unwrap().clone();
        let random_num: IpAddr = generate_random_ip();
        let res = client.post("https://test2.blockjoker.org/api/v1/missions")
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
            .header("Authorization", format!("Bearer {token}"))
            .header("Referer", "https://test2.blockjoker.org/home")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36 Edg/127.0.0.0")
            .header("Host", "test2.blockjoker.org")
            .header("Origin", "test2.blockjoker.org")
            .header("Connection", "Keep-Alive")
            .header("Content-Type", "application/json")
            .header("Cookie",current_session.to_string())
            .header("Client-Ip", &random_num.to_string())
            .header("X-Forwarded-For", &random_num.to_string())
            .header("Remote_Addr", &random_num.to_string())
            .send();
        match res {
            Ok(response) => {
                update_session(&session, &response);
                match response.text() {
                    Ok(text) => {
                        // println!("SALT 响应内容: {}", text);
                        if let Ok(json) = serde_json::from_str::<Value>(&text) {
                            if let Some(ok) = json["ok"].as_bool() {
                                if ok {
                                    if let Some(new_salt) = json["result"].as_str() {
                                        println!("更新salt: {}", new_salt);
                                        let mut salt_writer = salt.write().unwrap();
                                        *salt_writer = new_salt.to_string();
                                    }
                                } else {
                                    println!("更新salt");
                                }
                            }
                        } else {
                            println!("无法解析 JSON 响应");
                        }
                    },
                    Err(e) => {
                        println!("无法读取响应内容: {:?}", e);
                    }
                }
            },
            Err(e) => println!("更新salt失败: {:?}", e),
        }

        thread::sleep(Duration::from_secs(30));
    }
}

fn generate_single(salt: &str, rng: &mut rand::rngs::ThreadRng) -> Option<(String, String)> {
    let random_str: String = rng
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(48)
        .map(char::from)
        .collect();

    let ret = format!("{}{}", salt, random_str);
    let mut hasher = Sha256::new();
    hasher.update(ret);
    let result = hex::encode(hasher.finalize());

    if result.starts_with("00000") {
        Some((random_str, result))
    } else {
        None
    }
}

fn generate(salt: Arc<RwLock<String>>, found: Arc<AtomicBool>, token: String, session: Arc<RwLock<String>>) {
    let client = create_shared_client();
    let num_cores = num_cpus::get() / 2;
    let mut rng = rand::thread_rng();
    loop {
        let current_salt = salt.read().unwrap().clone();
        if current_salt.is_empty() {
            thread::sleep(Duration::from_secs(1));
            continue;
        }
        if current_salt != *salt.read().unwrap() {
            found.store(false, Ordering::Release);
            continue;
        }
        let current_session = session.read().unwrap().clone();
        if current_session.is_empty(){
            thread::sleep(Duration::from_secs(1));
            continue
        }
        if current_session != *session.read().unwrap() {
            found.store(false, Ordering::Release);
            continue;
        }
        let result = (0..num_cores)
            .into_iter()
            .find_map(|_| generate_single(&current_salt, &mut rng));

        if let Some((nonce, hash)) = result {
            if !found.load(Ordering::Acquire) {
                found.store(true, Ordering::Release);
                if current_salt != *salt.read().unwrap() {
                    found.store(false, Ordering::Release);
                    continue;
                }
                println!("尝试提交找到哈希：{},{},{}", current_salt, nonce, hash);
                let random_num: IpAddr = generate_random_ip();
                let res = client.post("https://test2.blockjoker.org/api/v1/missions/nonce")
                    .header("Accept", "application/json, text/plain, */*")
                    .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
                    .header("Authorization", format!("Bearer {token}"))
                    .header("Referer", "https://test2.blockjoker.org/home")
                    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36 Edg/127.0.0.0")
                    .header("Host", "test2.blockjoker.org")
                    .header("Connection", "Keep-Alive")
                    .header("Content-Type", "application/json")
                    .header("Origin", "test2.blockjoker.org")
                    .header("Cookie",current_session.to_string())
                    .header("Client-Ip", &random_num.to_string())
                    .header("X-Forwarded-For", &random_num.to_string())
                    .header("Remote_Addr", &random_num.to_string())
                    .json(&serde_json::json!({
                        "nonce": nonce
                    }))
                    .send();
                match res {
                    Ok(response) => {
                        update_session(&session, &response);
                        // println!("POST 响应状态码: {},salt：{}", response.status(), current_salt);
                        match response.text() {
                            Ok(text) => {
                                println!("POST 响应内容: {}", text);
                                if let Ok(json) = serde_json::from_str::<Value>(&text) {
                                    if let Some(ok) = json["ok"].as_bool() {
                                        if ok {
                                            if let Some(new_salt) = json["result"].as_str() {
                                                println!("<<<<<<成功提交哈希!>>>>>>");
                                                println!("读取到新salt: {}", new_salt);
                                                thread::sleep(Duration::from_secs(1));
                                                // let token_clone= token.clone();
                                                // let session_clone= session.clone();
                                                // fetch_point(token_clone, session_clone);
                                                // let token_clone= token.clone();
                                                // let session_clone= session.clone();
                                                // fetch_records(token_clone, session_clone);
                                                let mut salt_writer = salt.write().unwrap();
                                                *salt_writer = new_salt.to_string();
                                            }
                                        } else {
                                            println!("提交哈希失败，继续计算...");
                                        }
                                    }
                                } else {
                                    println!("无法解析 JSON 响应");
                                }
                            },
                            Err(e) => {
                                println!("无法读取响应内容: {:?}", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("POST 请求失败: {:?}", e);
                    }
                }
                found.store(false, Ordering::Release);
            }
            while found.load(Ordering::Acquire) {
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <token>", args[0]);
        std::process::exit(1);
    }
    let token = &args[1];

    let salt = Arc::new(RwLock::new(String::new()));
    let found = Arc::new(AtomicBool::new(false));
    let session = Arc::new(RwLock::new(String::new()));

    let num_cores = if args.len() > 2 {
        match args[2].parse::<usize>() {
            Ok(num) if num > 0 => num,
            _ => {
                eprintln!("线程数量填写错误.");
                num_cpus::get()
            }
        }
    } else {
        num_cpus::get()
    };
    println!("CPU核心数: {}", num_cores);

    let salt_clone = Arc::clone(&salt);
    let token_clone = token.clone();
    let session_clone = session.clone();
    thread::spawn(move || {
        fetch_salt(salt_clone, token_clone, session_clone);
    });


    let token_clone = token.clone();
    let session_clone = session.clone();
    thread::spawn(move || {
        fetch_point(token_clone, session_clone);
    });

    let token_clone = token.clone();
    let session_clone = session.clone();
    thread::spawn(move || {
        fetch_records(token_clone, session_clone);
    });

    println!("开始计算...");
    let mut handles = vec![];

    for _ in 0..num_cores {
        let salt_clone = Arc::clone(&salt);
        let found_clone = Arc::clone(&found);
        let token_clone = token.clone();
        let session_clone = session.clone();
        let handle = thread::spawn(move || {
            generate(salt_clone, found_clone, token_clone, session_clone);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }


}
