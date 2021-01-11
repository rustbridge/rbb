use std::env;

pub struct LinkPair {
  pub link: String,
  pub name: String,
}

pub struct Common {
  pub canonical_url: String,
  pub contact: Vec<LinkPair>,
  pub resources: Vec<LinkPair>,
  pub chapters: Vec<LinkPair>,
}

pub fn gen_common() -> Common {
  // TODO Source these from somewhere more accessible
  let contact = vec![
    LinkPair {
      link: "https://twitter.com/rustbridge".to_string(),
      name: "Twitter".to_string(),
    },
    LinkPair {
      link: "https://discord.gg/DpBApCd".to_string(),
      name: "Discord".to_string()
    },
    LinkPair {
      link: "https://github.com/rustbridge".to_string(),
      name: "GitHub".to_string()
    },
  ];
  let resources = vec![
    LinkPair {
      link: "https://www.rust-lang.org".to_string(),
      name: "Rust".to_string()
    },
    LinkPair {
      link: "https://github.com/rust-lang/rustlings".to_string(),
      name: "Rustlings".to_string()
    },
    LinkPair {
      link: "https://snake.rustbridge.com".to_string(),
      name: "Rusty Snake Book".to_string()
    },
  ];
  let chapters = vec![
    LinkPair {
      link: "https://berlin.rustbridge.com".to_string(),
      name: "RustBridge Berlin".to_string(),
    },
  ];

  Common {
    canonical_url: env::var("RBB_CANONICAL_URL").unwrap(),
    contact,
    resources, 
    chapters,
  }
}

pub fn preflight_env_check() {
  env::var("RBB_CANONICAL_URL").unwrap();
}