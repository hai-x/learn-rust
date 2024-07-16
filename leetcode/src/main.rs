use regex::Regex;
use std::{env, fs, io::Write, path::Path};

use fetch::{get_problem, Problem};

mod fetch;

pub mod problems;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("no question id provided!")
    }
    let id: u32 = args[1].parse().expect("question id parse error!");
    println!("Question id: {}\n🚀 Loading...", id);
    let existed_problems = get_existed_problems();
    if existed_problems.contains(&id) {
        panic!("problem already initialized!");
    }
    let problem = get_problem(id).await.expect("no problem!");
    let builder = Builder::new(problem);
    builder.build();
}

fn get_existed_problems() -> Vec<u32> {
    let content = fs::read_to_string("./src/problems/mod.rs").expect("no problem folder!");
    let id_pattern = Regex::new(r"p(\d{4})_").unwrap();
    id_pattern
        .captures_iter(&content)
        .map(|cap| cap.get(1).unwrap().as_str().parse::<u32>().unwrap())
        .collect()
}

struct Builder {
    problem: Option<Problem>,
}

impl Builder {
    fn new(problem: Problem) -> Self {
        Builder {
            problem: Some(problem),
        }
    }
    fn build(&self) {
        let problem = self
            .problem
            .as_ref()
            .expect("there is no problem to generate!");

        let file_name = format!(
            "p{:04}_{}",
            problem.question_id,
            problem.title_slug.replace("-", "_")
        );
        let file_path = Path::new("./src/problems").join(format!("{}.rs", file_name));
        if file_path.exists() {
            panic!("problem already initialized");
        }
        let template = fs::read_to_string("./m.rs").unwrap();

        let source = template
            .replace("__PROBLEM_TITLE__", &problem.title)
            .replace("__PROBLEM_DESC__", self.build_desc().as_str())
            .replace("__PROBLEM_DEFAULT_CODE__", self.build_return().as_str())
            .replace("__PROBLEM_ID__", &format!("{}", problem.question_id))
            .replace("__EXTRA_USE__", self.build_import().as_str())
            .replace("__PROBLEM_LINK__", self.build_problem_link().as_str())
            .replace("__DISCUSS_LINK__", self.build_discuss_link().as_str());

        let mut file1 = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path)
            .expect("write file1 failed!");

        file1.write_all(source.as_bytes()).unwrap();
        let mut file2 = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("./src/problems/mod.rs")
            .unwrap();
        writeln!(file2, "mod {};", file_name).expect("write file2 failed!");

        drop(file1);
        drop(file2);
    }

    fn build_desc(&self) -> String {
        let content = self.problem.as_ref().unwrap().content.as_str();

        content
            .replace("<strong>", "")
            .replace("</strong>", "")
            .replace("<em>", "")
            .replace("</em>", "")
            .replace("</p>", "")
            .replace("<p>", "")
            .replace("<b>", "")
            .replace("</b>", "")
            .replace("<pre>", "")
            .replace("</pre>", "")
            .replace("<ul>", "")
            .replace("</ul>", "")
            .replace("<li>", "")
            .replace("</li>", "")
            .replace("<code>", "")
            .replace("</code>", "")
            .replace("<i>", "")
            .replace("</i>", "")
            .replace("<sub>", "")
            .replace("</sub>", "")
            .replace("</sup>", "")
            .replace("<sup>", "^")
            .replace("&nbsp;", " ")
            .replace("&gt;", ">")
            .replace("&lt;", "<")
            .replace("&quot;", "\"")
            .replace("&minus;", "-")
            .replace("&#39;", "'")
            .replace("\n\n", "\n")
            .replace("\n", "\n * ")
    }

    fn build_return(&self) -> String {
        let return_type = self.problem.as_ref().unwrap().return_type.as_str();
        let code = self
            .problem
            .as_ref()
            .unwrap()
            .code_definition
            .iter()
            .find(|&d| d.value == "rust".to_string())
            .expect("there is no rust code to problem!");

        let code = &code.default_code;

        let re = Regex::new(r"\{[ \n]+}").unwrap();
        match return_type {
            "ListNode" => re
                .replace(&code, "{\n        Some(Box::new(ListNode::new(0)))\n    }")
                .to_string(),
            "ListNode[]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
            "TreeNode" => re
                .replace(
                    &code,
                    "{\n        Some(Rc::new(RefCell::new(TreeNode::new(0))))\n    }",
                )
                .to_string(),
            "boolean" => re.replace(&code, "{\n        false\n    }").to_string(),
            "character" => re.replace(&code, "{\n        '0'\n    }").to_string(),
            "character[][]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
            "double" => re.replace(&code, "{\n        0f64\n    }").to_string(),
            "double[]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
            "int[]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
            "integer" => re.replace(&code, "{\n        0\n    }").to_string(),
            "integer[]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
            "integer[][]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
            "list<String>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
            "list<TreeNode>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
            "list<boolean>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
            "list<double>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
            "list<integer>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
            "list<list<integer>>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
            "list<list<string>>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
            "list<string>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
            "null" => code.to_string(),
            "string" => re
                .replace(&code, "{\n        String::new()\n    }")
                .to_string(),
            "string[]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
            "void" => code.to_string(),
            "NestedInteger" => code.to_string(),
            "Node" => code.to_string(),
            _ => code.to_string(),
        }
    }

    fn build_import(&self) -> String {
        let code = self
            .problem
            .as_ref()
            .unwrap()
            .code_definition
            .iter()
            .find(|&d| d.value == "rust".to_string())
            .expect("there is no rust code to problem!");

        let code = &code.default_code;

        let mut extra_use_line = String::new();

        // a linked-list problem
        if code.contains("pub struct ListNode") {
            extra_use_line.push_str("\nuse crate::util::linked_list::{ListNode, to_list};")
        }
        if code.contains("pub struct TreeNode") {
            extra_use_line.push_str("\nuse crate::util::tree::{TreeNode, to_tree};")
        }
        if code.contains("pub struct Point") {
            extra_use_line.push_str("\nuse crate::util::point::Point;")
        }
        extra_use_line
    }

    fn build_problem_link(&self) -> String {
        format!(
            "https://leetcode.com/problems/{}/",
            self.problem.as_ref().unwrap().title_slug
        )
    }

    fn build_discuss_link(&self) -> String {
        format!(
            "https://leetcode.com/problems/{}/discuss/?currentPage=1&orderBy=most_votes&query=",
            self.problem.as_ref().unwrap().title_slug
        )
    }
}
