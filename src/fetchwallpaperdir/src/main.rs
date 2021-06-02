use clap::{App, Arg};
use rand::prelude::SliceRandom;
use rand::prelude::ThreadRng;
use serde_json::Value;
use core::panic;
use std::convert::TryInto;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
//use std::time::Instant;
//use std::time::Instant;



fn get_info() -> QueryInfo {
    
  
    let json_conf = read_json_from_file( format!("/home/{}/.RedditWallPaper/redditConfig.json",&whoami::username()));

    let subs =  match &json_conf["subreddits"]
    {
        Value::Array(subs) => subs
        , _ => panic!("cannot find subreddit in conf")
    };


    return QueryInfo {
        subreddit: match subs.choose(&mut ThreadRng::default())
        {
            Some(value) => match value.as_str()
            {
                Some(value) => value.to_string(),
                None => panic!("error"),
            },
            _ => panic!("error"),
        }
         ,
        sort: match json_conf["sort"].as_str() {
            Some(value) => value.to_string(),
            None => panic!("sd"),
        },
        query: match json_conf["match"].as_str() {
            Some(value) => value.to_string(),
            None => panic!("error"),
        },
        limit: match json_conf["limit"].as_i64() {
            Some(number) => match number.try_into()
            {
                Ok(num) =>num,
                Err(_) => panic!("limit to big"),
            },
            _ => panic!("not a number"),
        },
    };
}

#[tokio::main]
async fn main() {
   // let now = Instant::now();
    

    let mut arguments = read_args();
    let vb = get_info();

    

    arguments.sort = if arguments.sort != "" {
        arguments.sort
    } else {
        vb.sort.to_string()
    };
    arguments.query = if arguments.query != "" {
        arguments.query
    } else {
        vb.query.to_string()
    };
    arguments.limit = if arguments.limit > 0 {
        arguments.limit
    } else {
        match vb.limit.try_into() {
            Ok(number) => number,
            Err(_) => panic!("failed to convert to u8"),
        }
    };

    let url = if arguments.subreddit == "" && arguments.query != "" {
        create_url_no_sub(&arguments.query, &arguments.sort, &arguments.limit)
    } else {
        arguments.subreddit = if arguments.subreddit != "" {
            arguments.subreddit
        } else {
            vb.subreddit.to_string()
        };
        create_url_sub(
            &arguments.subreddit,
            &arguments.sort,
            &arguments.query,
            &arguments.limit,
        )
    };

  /*   println!("url is {}", url);
    println!("ns:{}", now.elapsed().as_nanos()); */
    println!("{}", get_pic_url(&url).await);
}


struct QueryInfo {
    subreddit: String,
    sort: String,
    query: String,
    limit: u8,
}
fn read_json_from_file<P: AsRef<Path>>(path: P) -> Value {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("cannot find json file"),
    };

    return match serde_json::from_reader(BufReader::new(file)) {
        Ok(value) => value,
        Err(_) => panic!("failed to read file"),
    };
}

fn create_url_sub(subreddit: &str, sort: &str, query: &str, limit: &u8) -> String {
    let mut url = format!(
        "https://reddit.com/r/{}/{}.json?limit={}",
        subreddit, sort, limit
    )
    .to_string();

    if query != "" {
        url.push_str(format!("&q={}", query).as_str());
    }
    return url;
}

fn create_url_no_sub(query: &str, sort: &str, limit: &u8) -> String {
    return format!(
        "https://reddit.com/search.json?q={}&sort={}&limit={}",
        query, sort, limit
    )
    .to_string();
}

async fn get_pic_url(url: &str) -> String {

    let root: Value = match reqwest::get(url).await {
        Ok(vec) => match vec.json().await {
            Ok(value) => value,
            Err(_) => panic!("failed to parse json"),
        },
        Err(_) => panic!("failed to get data"),
    };

    let posts = match root.pointer("/data/children") {
        Some(value) => match value.as_array() {
            Some(arr) => arr,
            None => panic!("failed to convert to array"),
        },
        None => panic!("failed to get children"),
    };

    let mut chosen_sub;
    let mut url;

    loop {
        chosen_sub = match posts.choose(&mut ThreadRng::default()) {
            Some(value) => value,
            _ => panic!("failed to pick a random post"),
        };

        url = match chosen_sub["data"]["url"].as_str() {
            Some(value) => value,
            _ => panic!("failed to get url"),
        };

        let _ = is_picture(url) && return url.to_string();
    }
}

fn is_picture(url: &str) -> bool {
    return url.ends_with(".jpg") || url.ends_with(".png");
}

fn read_args() -> QueryInfo {
    /*   let args: Vec<String> = env::args().collect();
    println!("{:?}", args); */
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        .arg(
            Arg::with_name("sub")
                .short("r")
                .long("sub")
                .takes_value(true)
                .help("A cool file"),
        )
        .arg(
            Arg::with_name("sort")
                .short("s")
                .long("sort")
                .takes_value(true)
                .help("Five less than your favorite number"),
        )
        .arg(
            Arg::with_name("match")
                .short("m")
                .long("match")
                .takes_value(true)
                .help("Five less than your favorite number"),
        )
        .arg(
            Arg::with_name("limit")
                .short("l")
                .long("limit")
                .takes_value(true)
                .help("Five less than your favorite number"),
        )
        .get_matches();

    return QueryInfo {
        subreddit: match matches.value_of("sub") {
            Some(x) => x,
            _ => "",
        }
        .to_string(),
        sort: match matches.value_of("sort") {
            Some(x) => x,
            _ => "",
        }
        .to_string(),
        query: match matches.value_of("match") {
            Some(x) => x,
            _ => "",
        }
        .to_string(),
        limit: match matches.value_of("limit") {
            Some(x) => match x.parse::<u8>() {
                Ok(_) => match x.parse::<u8>() {
                    Ok(result) => result,
                    Err(_) => panic!("argument at -l is not an integer"),
                },
                Err(_) => 0,
            },
            _ => 0,
        },
    };
}
