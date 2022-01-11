use std::env::current_dir;

#[test]
fn test_aggregator() {
    let project_dir = current_dir().unwrap().to_str().unwrap().to_string();
    let test_dir = format!("{}/{}", project_dir, "tests/fixtures");
    let test_result_files: Vec<String> = vec![
        format!("{}/google-score-test-1.json", test_dir),
        format!("{}/google-score-test-2.json", test_dir),
    ];
    let aggregate =
        lighthouse_groupie::create_result_aggregate("test", test_result_files, false).unwrap();

    let r = &aggregate["bestPractices"];

    assert_eq!(r.as_array().unwrap().len(), 2);

    let v = r
        .as_array()
        .unwrap()
        .clone()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(v[0] as i32, 1);

    let r = &aggregate["bootupTime"];

    let v = r
        .as_array()
        .unwrap()
        .clone()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(v[0] as i32, 70);

    let r = &aggregate["firstMeaningfulPaint"];

    let v = r
        .as_array()
        .unwrap()
        .clone()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(v[0] as i32, 1143);
}
