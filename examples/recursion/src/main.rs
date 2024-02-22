use serde_json::{json, Value};

fn main() {
    let a = ['a', 'b', 'c'];

    // let mut iter = a.iter().enumerate();

    // assert_eq!(iter.next(), Some((0, &'a')));
    // assert_eq!(iter.next(), Some((1, &'b')));
    // assert_eq!(iter.next(), Some((2, &'c')));
    // assert_eq!(iter.next(), None);

    for (x, y) in a.iter().enumerate() {
        println!("index {x}, value {y}");
    }

    let x = why_why_analyses(
        &json!([
          {
            "why": "Provided reason of cause 1.",
            "reasons": [
              {
                "why": "Provided reason of cause 1.1.",
                "reasons": [
                    {
                        "why": "Provided reason of cause 1.1.1.",
                        "reasons": []
                    },
                    {
                        "why": "Provided reason of cause 1.1.2.",
                        "reasons": []
                    }
                ]
              },
              {
                "why": "Provided reason of cause 1.2.",
                "reasons": []
              }
            ]
          },
          {
            "why": "Provided reason of cause 2.",
            "reasons": [
              {
                "why": "Provided reason of cause 2.1.",
                "reasons": [
                    {
                        "why": "Provided reason of cause 2.1.1.",
                        "reasons":  [
                            "Provided reason of cause 2.1.1.1.",
                            "Provided reason of cause 2.1.1.2.",
                        ],

                    },
                ]
              },
              {
                "why": "Provided reason of cause 2.2",
                "reasons": []
              }
            ]
          }
        ]),
        None,
    );
    println!("{x:#?}");
}

fn why_why_analyses(value: &Value, prefix: Option<&str>) -> Vec<String> {
    let base_prefix = prefix.unwrap_or("");
    let mut analyses = vec![];

    if let Some(causes) = value.as_array() {
        for (i, cause) in causes.iter().enumerate() {
            let new_prefix = format!("{base_prefix}{prefix}.", prefix = i + 1);
            let why = cause["why"].as_str().unwrap_or("(Empty Why)");

            if why != "(Empty Why)" {
                analyses.push(format!("{new_prefix} {why}"));
            } else if why == "(Empty Why)" {
                println!("Is empty why {:?}", cause);
                if let Value::String(reason_cause) = cause {
                    analyses.push(format!("{new_prefix} {}", &reason_cause.as_str()));
                }
                // analyses.push(format!("{new_prefix} {:?}", cause["reasons"]));
                // if let Value::Array(reasons) = &cause["reasons"] {
                //     let mut reason_prefix = new_prefix.clone();
                //     println!("REASONS: {:?}", reasons);
                // for (i, reason) in reasons.iter().enumerate() {
                //     reason_prefix = format!("{reason_prefix}{prefix}.", prefix = i + 1);
                //     analyses.push(format!("{reason_prefix} {:?}", reason));
                // }

                // }
            }

            let nested_analyses = why_why_analyses(&cause["reasons"], Some(new_prefix.as_str()));
            analyses.extend(nested_analyses);
        }
    }

    analyses
}

#[cfg(test)]
mod why_why_analyses_test {
    use super::*;
    #[test]
    fn check_why_why_analyses() {
        let x = why_why_analyses(
            &json!([
              {
                "why": "Provided reason of cause 1.",
                "reasons": [
                  {
                    "why": "Provided reason of cause 1.1.",
                    "reasons": [
                        {
                            "why": "Provided reason of cause 1.1.1.",
                            "reasons": []
                        },
                        {
                            "why": "Provided reason of cause 1.1.2.",
                            "reasons": []
                        }
                    ]
                  },
                  {
                    "why": "Provided reason of cause 1.2.",
                    "reasons": []
                  }
                ]
              },
              {
                "why": "Provided reason of cause 2.",
                "reasons": [
                  {
                    "why": "Provided reason of cause 2.1.",
                    "reasons": [
                        {
                            "why": "Provided reason of cause 2.1.1.",
                            "reasons":  [
                                "Provided reason of cause 2.1.1.1.",
                                "Provided reason of cause 2.1.1.2.",
                            ],

                        },
                    ]
                  },
                  {
                    "why": "Provided reason of cause 2.2",
                    "reasons": []
                  }
                ]
              }
            ]),
            None,
        );
        let expected = vec![
            "1. Provided reason of cause 1.",
            "1.1. Provided reason of cause 1.1.",
            "1.1.1. Provided reason of cause 1.1.1.",
            "1.1.2. Provided reason of cause 1.1.2.",
            "1.2. Provided reason of cause 1.2.",
            "2. Provided reason of cause 2.",
            "2.1. Provided reason of cause 2.1.",
            "2.1.1. Provided reason of cause 2.1.1.",
            "2.1.1.1. Provided reason of cause 2.1.1.1.",
            "2.1.1.2. Provided reason of cause 2.1.1.2.",
            "2.2. Provided reason of cause 2.2",
        ];
        assert_eq!(x, expected);
        // panic!("force fail to see output");
    }
}