
#[test]
fn test_boolean_3_0_one_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one < two\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"two\":2,\"three\":3}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_3_1_one_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one <= two\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"two\":2,\"three\":3}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_3_2_one_one() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one == one\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"two\":2,\"three\":3}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_3_3_one_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one == two\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"two\":2,\"three\":3}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_3_4_one_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one > two\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"two\":2,\"three\":3}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_3_5_one_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one >= two\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"two\":2,\"three\":3}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_3_6_one_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one != two\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"two\":2,\"three\":3}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_3_7_one_two_three_one() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one < two && three > one\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"two\":2,\"three\":3}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_3_8_one_two_three_one() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one < two || three > one\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"two\":2,\"three\":3}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_3_9_one_two_three_one() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one < two || three < one\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"two\":2,\"three\":3}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_3_10_two_one_three_one() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"two < one || three < one\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"two\":2,\"three\":3}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_0_true_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"True && False\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_1_false_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"False && True\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_2_true_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"True && True\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_3_false_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"False && False\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_4_true_number() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"True && Number\",\"result\":5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_5_number_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number && True\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_6_number_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number && False\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_7_number_emptylist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number && EmptyList\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_8_number_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number && True\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_9_emptylist_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"EmptyList && True\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_10_emptylist_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"EmptyList && False\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_11_true_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"True || False\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_12_true_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"True || True\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_13_false_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"False || True\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_14_false_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"False || False\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_15_number_emptylist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number || EmptyList\",\"result\":5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_16_number_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number || True\",\"result\":5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_17_number_true_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number || True && False\",\"result\":5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_18_number_true_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"(Number || True) && False\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_19_number_true_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number || (True && False)\",\"result\":5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_20_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!True\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_21_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!False\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_22_number() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!Number\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_23_emptylist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!EmptyList\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_24_true_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"True && !False\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_25_true_emptylist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"True && !EmptyList\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_26_false_emptylist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!False && !EmptyList\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_27_true_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!(True && False)\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_28_zero() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!Zero\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_4_29_zero() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!!Zero\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"True\":true,\"False\":false,\"Number\":5,\"EmptyList\":[],\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_5_0_outer_empty_string_outer_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.empty_string || outer.foo\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"foo\":\"foo\",\"bool\":false,\"empty_list\":[],\"empty_string\":\"\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_5_1_outer_nokey_outer_bool_ou() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.nokey || outer.bool || outer.empty_list || outer.empty_string || outer.foo\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"foo\":\"foo\",\"bool\":false,\"empty_list\":[],\"empty_string\":\"\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_6_0_outer_foo_outer_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.foo || outer.bar\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"foo\":\"foo\",\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_6_1_outer_foo_outer_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.foo||outer.bar\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"foo\":\"foo\",\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_6_2_outer_bar_outer_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.bar || outer.baz\",\"result\":\"bar\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"foo\":\"foo\",\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_6_3_outer_bar_outer_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.bar||outer.baz\",\"result\":\"bar\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"foo\":\"foo\",\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_6_4_outer_bad_outer_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.bad || outer.foo\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"foo\":\"foo\",\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_6_5_outer_bad_outer_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.bad||outer.foo\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"foo\":\"foo\",\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_6_6_outer_foo_outer_bad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.foo || outer.bad\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"foo\":\"foo\",\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_6_7_outer_foo_outer_bad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.foo||outer.bad\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"foo\":\"foo\",\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_6_8_outer_bad_outer_alsobad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.bad || outer.alsobad\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"foo\":\"foo\",\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_6_9_outer_bad_outer_alsobad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.bad||outer.alsobad\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"foo\":\"foo\",\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_escape_7_0_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"foo.bar\\\"\",\"result\":\"dot\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo.bar\":\"dot\",\"foo bar\":\"space\",\"foo\\nbar\":\"newline\",\"foo\\\"bar\":\"doublequote\",\"c:\\\\\\\\windows\\\\path\":\"windows\",\"/unix/path\":\"unix\",\"\\\"\\\"\\\"\":\"threequotes\",\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_escape_7_1_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"foo bar\\\"\",\"result\":\"space\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo.bar\":\"dot\",\"foo bar\":\"space\",\"foo\\nbar\":\"newline\",\"foo\\\"bar\":\"doublequote\",\"c:\\\\\\\\windows\\\\path\":\"windows\",\"/unix/path\":\"unix\",\"\\\"\\\"\\\"\":\"threequotes\",\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_escape_7_2_foo_nbar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"foo\\\\nbar\\\"\",\"result\":\"newline\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo.bar\":\"dot\",\"foo bar\":\"space\",\"foo\\nbar\":\"newline\",\"foo\\\"bar\":\"doublequote\",\"c:\\\\\\\\windows\\\\path\":\"windows\",\"/unix/path\":\"unix\",\"\\\"\\\"\\\"\":\"threequotes\",\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_escape_7_3_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"foo\\\\\\\"bar\\\"\",\"result\":\"doublequote\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo.bar\":\"dot\",\"foo bar\":\"space\",\"foo\\nbar\":\"newline\",\"foo\\\"bar\":\"doublequote\",\"c:\\\\\\\\windows\\\\path\":\"windows\",\"/unix/path\":\"unix\",\"\\\"\\\"\\\"\":\"threequotes\",\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_escape_7_4_c_windows_path() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"c:\\\\\\\\\\\\\\\\windows\\\\\\\\path\\\"\",\"result\":\"windows\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo.bar\":\"dot\",\"foo bar\":\"space\",\"foo\\nbar\":\"newline\",\"foo\\\"bar\":\"doublequote\",\"c:\\\\\\\\windows\\\\path\":\"windows\",\"/unix/path\":\"unix\",\"\\\"\\\"\\\"\":\"threequotes\",\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_escape_7_5_unix_path() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"/unix/path\\\"\",\"result\":\"unix\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo.bar\":\"dot\",\"foo bar\":\"space\",\"foo\\nbar\":\"newline\",\"foo\\\"bar\":\"doublequote\",\"c:\\\\\\\\windows\\\\path\":\"windows\",\"/unix/path\":\"unix\",\"\\\"\\\"\\\"\":\"threequotes\",\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_escape_7_6_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\"\\\\\\\"\\\\\\\"\\\"\",\"result\":\"threequotes\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo.bar\":\"dot\",\"foo bar\":\"space\",\"foo\\nbar\":\"newline\",\"foo\\\"bar\":\"doublequote\",\"c:\\\\\\\\windows\\\\path\":\"windows\",\"/unix/path\":\"unix\",\"\\\"\\\"\\\"\":\"threequotes\",\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_escape_7_7_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"bar\\\".\\\"baz\\\"\",\"result\":\"qux\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo.bar\":\"dot\",\"foo bar\":\"space\",\"foo\\nbar\":\"newline\",\"foo\\\"bar\":\"doublequote\",\"c:\\\\\\\\windows\\\\path\":\"windows\",\"/unix/path\":\"unix\",\"\\\"\\\"\\\"\":\"threequotes\",\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_wildcard_8_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*[0]\",\"result\":[0,0]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"a\":[0,1,2],\"b\":[0,1,2]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_9_0_string() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"string.*\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"val\",\"bar\":\"val\"},\"number\":23,\"array\":[1,2,3],\"nullvalue\":null}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_9_1_hash() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"hash.*\",\"result\":[\"val\",\"val\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"val\",\"bar\":\"val\"},\"number\":23,\"array\":[1,2,3],\"nullvalue\":null}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_9_2_number() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"number.*\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"val\",\"bar\":\"val\"},\"number\":23,\"array\":[1,2,3],\"nullvalue\":null}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_9_3_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"array.*\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"val\",\"bar\":\"val\"},\"number\":23,\"array\":[1,2,3],\"nullvalue\":null}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_9_4_nullvalue() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"nullvalue.*\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"val\",\"bar\":\"val\"},\"number\":23,\"array\":[1,2,3],\"nullvalue\":null}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_10_0_string() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"string[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_10_1_hash() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"hash[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_10_2_number() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"number[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_10_3_nullvalue() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"nullvalue[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_10_4_string_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"string[*].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_10_5_hash_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"hash[*].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_10_6_number_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"number[*].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_10_7_nullvalue_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"nullvalue[*].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_10_8_nullvalue_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"nullvalue[*].foo[*].bar\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_11_0_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][0]\",\"result\":[[\"one\",\"two\"],[\"five\",\"six\"],[\"nine\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_11_1_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][1]\",\"result\":[[\"three\",\"four\"],[\"seven\",\"eight\"],[\"ten\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_11_2_foo_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][0][0]\",\"result\":[\"one\",\"five\",\"nine\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_11_3_foo_1_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][1][0]\",\"result\":[\"three\",\"seven\",\"ten\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_11_4_foo_0_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][0][1]\",\"result\":[\"two\",\"six\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_11_5_foo_1_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][1][1]\",\"result\":[\"four\",\"eight\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_11_6_foo_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][2]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_11_7_foo_2_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][2][2]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_11_8_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bar[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_11_9_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bar[*].baz[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_12_0_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][0]\",\"result\":[\"one\",\"three\",\"five\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[\"one\",\"two\"],[\"three\",\"four\"],[\"five\"]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_12_1_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][1]\",\"result\":[\"two\",\"four\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[\"one\",\"two\"],[\"three\",\"four\"],[\"five\"]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_13_0_foo_bar_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar[0]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[]},{\"bar\":[]},{\"bar\":[]}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_14_0_foo_bar_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar[0]\",\"result\":[\"one\",\"three\",\"five\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[\"one\",\"two\"]},{\"bar\":[\"three\",\"four\"]},{\"bar\":[\"five\"]}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_14_1_foo_bar_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar[1]\",\"result\":[\"two\",\"four\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[\"one\",\"two\"]},{\"bar\":[\"three\",\"four\"]},{\"bar\":[\"five\"]}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_14_2_foo_bar_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar[2]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[\"one\",\"two\"]},{\"bar\":[\"three\",\"four\"]},{\"bar\":[\"five\"]}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_15_0_foo_bar_kind() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar.kind\",\"result\":[\"basic\",\"intermediate\",\"advanced\",\"expert\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":{\"kind\":\"basic\"}},{\"bar\":{\"kind\":\"intermediate\"}},{\"bar\":{\"kind\":\"advanced\"}},{\"bar\":{\"kind\":\"expert\"}},{\"bar\":\"string\"}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_16_0_foo_bar_kind() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar[*].kind\",\"result\":[[\"basic\",\"intermediate\"],[\"advanced\",\"expert\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"kind\":\"basic\"},{\"kind\":\"intermediate\"}]},{\"bar\":[{\"kind\":\"advanced\"},{\"kind\":\"expert\"}]},{\"bar\":\"string\"}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_16_1_foo_bar_0_kind() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar[0].kind\",\"result\":[\"basic\",\"advanced\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"kind\":\"basic\"},{\"kind\":\"intermediate\"}]},{\"bar\":[{\"kind\":\"advanced\"},{\"kind\":\"expert\"}]},{\"bar\":\"string\"}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_17_0_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[*]\",\"result\":[[\"one\",\"two\"],[\"three\",\"four\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[[\"one\",\"two\"],[\"three\",\"four\"]]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_17_1_foo_bar_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[0]\",\"result\":[\"one\",\"two\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[[\"one\",\"two\"],[\"three\",\"four\"]]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_17_2_foo_bar_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[0][0]\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[[\"one\",\"two\"],[\"three\",\"four\"]]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_17_3_foo_bar_0_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[0][0][0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[[\"one\",\"two\"],[\"three\",\"four\"]]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_17_4_foo_bar_0_0_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[0][0][0][0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[[\"one\",\"two\"],[\"three\",\"four\"]]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_17_5_foo_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0][0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[[\"one\",\"two\"],[\"three\",\"four\"]]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_18_0_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[*].baz\",\"result\":[[\"one\",\"two\",\"three\"],[\"four\",\"five\",\"six\"],[\"seven\",\"eight\",\"nine\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[{\"baz\":[\"one\",\"two\",\"three\"]},{\"baz\":[\"four\",\"five\",\"six\"]},{\"baz\":[\"seven\",\"eight\",\"nine\"]}]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_18_1_foo_bar_baz_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[*].baz[0]\",\"result\":[\"one\",\"four\",\"seven\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[{\"baz\":[\"one\",\"two\",\"three\"]},{\"baz\":[\"four\",\"five\",\"six\"]},{\"baz\":[\"seven\",\"eight\",\"nine\"]}]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_18_2_foo_bar_baz_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[*].baz[1]\",\"result\":[\"two\",\"five\",\"eight\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[{\"baz\":[\"one\",\"two\",\"three\"]},{\"baz\":[\"four\",\"five\",\"six\"]},{\"baz\":[\"seven\",\"eight\",\"nine\"]}]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_18_3_foo_bar_baz_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[*].baz[2]\",\"result\":[\"three\",\"six\",\"nine\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[{\"baz\":[\"one\",\"two\",\"three\"]},{\"baz\":[\"four\",\"five\",\"six\"]},{\"baz\":[\"seven\",\"eight\",\"nine\"]}]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_18_4_foo_bar_baz_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[*].baz[3]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[{\"baz\":[\"one\",\"two\",\"three\"]},{\"baz\":[\"four\",\"five\",\"six\"]},{\"baz\":[\"seven\",\"eight\",\"nine\"]}]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_19_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[*]\",\"result\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_19_1_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[*].bar\",\"result\":[\"one\",\"two\",\"three\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_19_2_notbar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[*].notbar\",\"result\":[\"four\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_20_0_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar\",\"result\":[\"one\",\"two\",\"three\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_20_1_foo_notbar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].notbar\",\"result\":[\"four\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_21_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*\",\"result\":[{\"sub1\":{\"foo\":\"one\"}},{\"sub1\":{\"foo\":\"one\"}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"top1\":{\"sub1\":{\"foo\":\"one\"}},\"top2\":{\"sub1\":{\"foo\":\"one\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_21_1_sub1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.sub1\",\"result\":[{\"foo\":\"one\"},{\"foo\":\"one\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"top1\":{\"sub1\":{\"foo\":\"one\"}},\"top2\":{\"sub1\":{\"foo\":\"one\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_21_2_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.*\",\"result\":[[{\"foo\":\"one\"}],[{\"foo\":\"one\"}]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"top1\":{\"sub1\":{\"foo\":\"one\"}},\"top2\":{\"sub1\":{\"foo\":\"one\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_21_3_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.*.foo[]\",\"result\":[\"one\",\"one\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"top1\":{\"sub1\":{\"foo\":\"one\"}},\"top2\":{\"sub1\":{\"foo\":\"one\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_21_4_sub1_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.sub1.foo\",\"result\":[\"one\",\"one\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"top1\":{\"sub1\":{\"foo\":\"one\"}},\"top2\":{\"sub1\":{\"foo\":\"one\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_22_0_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.bar\",\"result\":[\"one\",\"one\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"one\"},\"other\":{\"bar\":\"one\"},\"nomatch\":{\"notbar\":\"three\"}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_23_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*\",\"result\":[{\"second-1\":\"val\"},{\"second-1\":\"val\"},{\"second-1\":\"val\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"first-1\":{\"second-1\":\"val\"},\"first-2\":{\"second-1\":\"val\"},\"first-3\":{\"second-1\":\"val\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_23_1_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.*\",\"result\":[[\"val\"],[\"val\"],[\"val\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"first-1\":{\"second-1\":\"val\"},\"first-2\":{\"second-1\":\"val\"},\"first-3\":{\"second-1\":\"val\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_23_2_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.*.*\",\"result\":[[],[],[]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"first-1\":{\"second-1\":\"val\"},\"first-2\":{\"second-1\":\"val\"},\"first-3\":{\"second-1\":\"val\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_23_3_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.*.*.*\",\"result\":[[],[],[]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"first-1\":{\"second-1\":\"val\"},\"first-2\":{\"second-1\":\"val\"},\"first-3\":{\"second-1\":\"val\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_24_0_foo_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.baz\",\"result\":[\"val\",\"val\",\"val\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"val\"},\"other\":{\"baz\":\"val\"},\"other2\":{\"baz\":\"val\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other5\":{\"other\":{\"a\":1,\"b\":1,\"c\":1}}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_24_1_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar.*\",\"result\":[\"val\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"val\"},\"other\":{\"baz\":\"val\"},\"other2\":{\"baz\":\"val\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other5\":{\"other\":{\"a\":1,\"b\":1,\"c\":1}}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_24_2_foo_notbaz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.notbaz\",\"result\":[[\"a\",\"b\",\"c\"],[\"a\",\"b\",\"c\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"val\"},\"other\":{\"baz\":\"val\"},\"other2\":{\"baz\":\"val\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other5\":{\"other\":{\"a\":1,\"b\":1,\"c\":1}}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_24_3_foo_notbaz_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.notbaz[0]\",\"result\":[\"a\",\"a\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"val\"},\"other\":{\"baz\":\"val\"},\"other2\":{\"baz\":\"val\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other5\":{\"other\":{\"a\":1,\"b\":1,\"c\":1}}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_24_4_foo_notbaz_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.notbaz[-1]\",\"result\":[\"c\",\"c\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"val\"},\"other\":{\"baz\":\"val\"},\"other2\":{\"baz\":\"val\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other5\":{\"other\":{\"a\":1,\"b\":1,\"c\":1}}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_literal_25_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'foo'\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_25_1_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'  foo  '\",\"result\":\"  foo  \"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_25_2_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'0'\",\"result\":\"0\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_25_3_newline() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'newline\\n'\",\"result\":\"newline\\n\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_25_4_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'\\n'\",\"result\":\"\\n\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_25_5_ok() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'✓'\",\"result\":\"✓\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_25_6_g() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'𝄞'\",\"result\":\"𝄞\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_25_7_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'  [foo]  '\",\"result\":\"  [foo]  \"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_25_8_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'[foo]'\",\"result\":\"[foo]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_25_9_do_not_interpret_escaped_() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Do not interpret escaped unicode.\",\"expression\":\"'\\\\u03a6'\",\"result\":\"\\\\u03a6\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_25_10_can_escape_the_single_quo() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Can escape the single quote\",\"expression\":\"'foo\\\\'bar'\",\"result\":\"foo'bar\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_25_11_backslash_not_followed_by() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Backslash not followed by single quote is treated as any other character\",\"expression\":\"'\\\\z'\",\"result\":\"\\\\z\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_25_12_backslash_not_followed_by() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Backslash not followed by single quote is treated as any other character\",\"expression\":\"'\\\\\\\\'\",\"result\":\"\\\\\\\\\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_26_0_literal_with_leading_whit() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Literal with leading whitespace\",\"expression\":\"`  {\\\"foo\\\": true}`\",\"result\":{\"foo\":true}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_26_1_literal_with_trailing_whi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Literal with trailing whitespace\",\"expression\":\"`{\\\"foo\\\": true}   `\",\"result\":{\"foo\":true}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_26_2_literal_on_rhs_of_subexpr() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Literal on RHS of subexpr not allowed\",\"expression\":\"foo.`\\\"bar\\\"`\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`\\\"foo\\\"`\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_1_interpret_escaped_unicode() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Interpret escaped unicode.\",\"expression\":\"`\\\"\\\\u03a6\\\"`\",\"result\":\"Φ\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_2_ok() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`\\\"✓\\\"`\",\"result\":\"✓\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_3_1_2_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`[1, 2, 3]`\",\"result\":[1,2,3]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_4_a_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`{\\\"a\\\": \\\"b\\\"}`\",\"result\":{\"a\":\"b\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_5_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`true`\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_6_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`false`\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_7_null() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`null`\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_8_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`0`\",\"result\":0}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_9_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`1`\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_10_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`2`\",\"result\":2}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_11_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`3`\",\"result\":3}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_12_4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`4`\",\"result\":4}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_13_5() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`5`\",\"result\":5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_14_6() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`6`\",\"result\":6}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_15_7() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`7`\",\"result\":7}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_16_8() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`8`\",\"result\":8}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_17_9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`9`\",\"result\":9}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_18_escaping_a_backtick_in_qu() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Escaping a backtick in quotes\",\"expression\":\"`\\\"foo\\\\`bar\\\"`\",\"result\":\"foo`bar\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_19_double_quote_in_literal() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Double quote in literal\",\"expression\":\"`\\\"foo\\\\\\\"bar\\\"`\",\"result\":\"foo\\\"bar\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_20_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`\\\"1\\\\`\\\"`\",\"result\":\"1`\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_21_multiple_literal_expressi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multiple literal expressions with escapes\",\"expression\":\"`\\\"\\\\\\\\\\\"`.{a:`\\\"b\\\"`}\",\"result\":{\"a\":\"b\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_22_literal_identifier() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"literal . identifier\",\"expression\":\"`{\\\"a\\\": \\\"b\\\"}`.a\",\"result\":\"b\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_23_literal_identifier_identi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"literal . identifier . identifier\",\"expression\":\"`{\\\"a\\\": {\\\"b\\\": \\\"c\\\"}}`.a.b\",\"result\":\"c\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_27_24_literal_identifier_bracke() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"literal . identifier bracket-expr\",\"expression\":\"`[0, 1, 2]`[1]\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_slice_28_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[:]\",\"result\":[{\"a\":1},{\"a\":2},{\"a\":3}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[{\"a\":1},{\"a\":2},{\"a\":3}]").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_28_1_2_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[:2].a\",\"result\":[1,2]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[{\"a\":1},{\"a\":2},{\"a\":3}]").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_28_2_1_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[::-1].a\",\"result\":[3,2,1]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[{\"a\":1},{\"a\":2},{\"a\":3}]").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_28_3_2_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[:2].b\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[{\"a\":1},{\"a\":2},{\"a\":3}]").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_29_0_foo_2_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[:2].a\",\"result\":[1,2]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1},{\"a\":2},{\"a\":3}],\"bar\":[{\"a\":{\"b\":1}},{\"a\":{\"b\":2}},{\"a\":{\"b\":3}}],\"baz\":50}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_29_1_foo_2_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[:2].b\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1},{\"a\":2},{\"a\":3}],\"bar\":[{\"a\":{\"b\":1}},{\"a\":{\"b\":2}},{\"a\":{\"b\":3}}],\"baz\":50}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_29_2_foo_2_a_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[:2].a.b\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1},{\"a\":2},{\"a\":3}],\"bar\":[{\"a\":{\"b\":1}},{\"a\":{\"b\":2}},{\"a\":{\"b\":3}}],\"baz\":50}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_29_3_bar_1_a_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bar[::-1].a.b\",\"result\":[3,2,1]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1},{\"a\":2},{\"a\":3}],\"bar\":[{\"a\":{\"b\":1}},{\"a\":{\"b\":2}},{\"a\":{\"b\":3}}],\"baz\":50}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_29_4_bar_2_a_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bar[:2].a.b\",\"result\":[1,2]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1},{\"a\":2},{\"a\":3}],\"bar\":[{\"a\":{\"b\":1}},{\"a\":{\"b\":2}},{\"a\":{\"b\":3}}],\"baz\":50}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_29_5_baz_2_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"baz[:2].a\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1},{\"a\":2},{\"a\":3}],\"bar\":[{\"a\":{\"b\":1}},{\"a\":{\"b\":2}},{\"a\":{\"b\":3}}],\"baz\":50}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_0_bar_0_10() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bar[0:10]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_1_foo_0_10_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0:10:1]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_2_foo_0_10() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0:10]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_3_foo_0_10() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0:10:]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_4_foo_0_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0::1]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_5_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0::]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_6_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0:]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_7_foo_10_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[:10:1]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_8_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[::1]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_9_foo_10() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[:10:]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_10_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[::]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_11_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[:]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_12_foo_1_9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[1:9]\",\"result\":[1,2,3,4,5,6,7,8]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_13_foo_0_10_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0:10:2]\",\"result\":[0,2,4,6,8]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_14_foo_5() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[5:]\",\"result\":[5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_15_foo_5_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[5::2]\",\"result\":[5,7,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_16_foo_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[::2]\",\"result\":[0,2,4,6,8]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_17_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[::-1]\",\"result\":[9,8,7,6,5,4,3,2,1,0]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_18_foo_1_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[1::2]\",\"result\":[1,3,5,7,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_19_foo_10_0_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[10:0:-1]\",\"result\":[9,8,7,6,5,4,3,2,1]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_20_foo_10_5_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[10:5:-1]\",\"result\":[9,8,7,6]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_21_foo_8_2_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[8:2:-2]\",\"result\":[8,6,4]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_22_foo_0_20() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0:20]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_23_foo_10_20_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[10:-20:-1]\",\"result\":[9,8,7,6,5,4,3,2,1,0]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_24_foo_10_20() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[10:-20]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_25_foo_4_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[-4:-1]\",\"result\":[6,7,8]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_26_foo_5_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[:-5:-1]\",\"result\":[9,8,7,6]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_27_foo_8_2_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[8:2:0]\",\"error\":\"invalid-value\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_28_foo_8_2_0_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[8:2:0:1]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_29_foo_8_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[8:2&]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_30_30_foo_2_a_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[2:a:3]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9],\"bar\":{\"baz\":1}}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_pipe_31_0_foo_bar_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar[*] | [0][0]\",\"result\":{\"baz\":\"one\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"baz\":\"one\"},{\"baz\":\"two\"}]},{\"bar\":[{\"baz\":\"three\"},{\"baz\":\"four\"}]}]}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_32_0_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo | bar\",\"result\":{\"baz\":\"one\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_32_1_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo | bar | baz\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_32_2_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo|bar| baz\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_32_3_not_there_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"not_there | [0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_32_4_not_there_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"not_there | [0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_32_5_foo_bar_foo_other_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[foo.bar, foo.other] | [0]\",\"result\":{\"baz\":\"one\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_32_6_a_foo_bar_b_foo_other_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{\\\"a\\\": foo.bar, \\\"b\\\": foo.other} | a\",\"result\":{\"baz\":\"one\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_32_7_a_foo_bar_b_foo_other_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{\\\"a\\\": foo.bar, \\\"b\\\": foo.other} | b\",\"result\":{\"baz\":\"two\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_32_8_foo_bam_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bam || foo.bar | baz\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_32_9_foo_not_there_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo | not_there || bar\",\"result\":{\"baz\":\"one\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_33_0_foo_baz_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.baz | [0]\",\"result\":\"subkey\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"subkey\"},\"other\":{\"baz\":\"subkey\"},\"other2\":{\"baz\":\"subkey\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_33_1_foo_baz_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.baz | [1]\",\"result\":\"subkey\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"subkey\"},\"other\":{\"baz\":\"subkey\"},\"other2\":{\"baz\":\"subkey\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_33_2_foo_baz_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.baz | [2]\",\"result\":\"subkey\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"subkey\"},\"other\":{\"baz\":\"subkey\"},\"other2\":{\"baz\":\"subkey\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_33_3_foo_bar_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar.* | [0]\",\"result\":\"subkey\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"subkey\"},\"other\":{\"baz\":\"subkey\"},\"other2\":{\"baz\":\"subkey\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_33_4_foo_notbaz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.notbaz | [*]\",\"result\":[[\"a\",\"b\",\"c\"],[\"a\",\"b\",\"c\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"subkey\"},\"other\":{\"baz\":\"subkey\"},\"other2\":{\"baz\":\"subkey\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_33_5_a_foo_bar_b_foo_other_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{\\\"a\\\": foo.bar, \\\"b\\\": foo.other} | *.baz\",\"result\":[\"subkey\",\"subkey\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"subkey\"},\"other\":{\"baz\":\"subkey\"},\"other2\":{\"baz\":\"subkey\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_unicode_34_0_snowman() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"☃\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"☃\":true}").unwrap());
    case.assert("unicode", data).unwrap();
}


#[test]
fn test_unicode_35_0_heart() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"♪♫•*¨*•.¸¸❤¸¸.•*¨*•♫♪\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"♪♫•*¨*•.¸¸❤¸¸.•*¨*•♫♪\":true}").unwrap());
    case.assert("unicode", data).unwrap();
}


#[test]
fn test_unicode_36_0_yin_yang() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"☯\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"☯\":true}").unwrap());
    case.assert("unicode", data).unwrap();
}


#[test]
fn test_unicode_37_0_foo_ok() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].\\\"✓\\\"\",\"result\":[\"✓\",\"✗\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"✓\":\"✓\"},{\"✓\":\"✗\"}]}").unwrap());
    case.assert("unicode", data).unwrap();
}


#[test]
fn test_indices_38_0_string() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"string[]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_38_1_hash() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"hash[]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_38_2_number() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"number[]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_38_3_nullvalue() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"nullvalue[]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_38_4_string_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"string[].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_38_5_hash_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"hash[].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_38_6_number_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"number[].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_38_7_nullvalue_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"nullvalue[].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_38_8_nullvalue_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"nullvalue[].foo[].bar\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"string\":\"string\",\"hash\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"number\":23,\"nullvalue\":null}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_39_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo\",\"result\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_39_1_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[]\",\"result\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_39_2_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar\",\"result\":[[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}],[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_39_3_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar[]\",\"result\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3},{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_39_4_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar[].baz\",\"result\":[1,3,5,7]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_40_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[]\",\"result\":[[\"one\",\"two\"],[\"three\",\"four\"],[\"five\",\"six\"],[\"seven\",\"eight\"],[\"nine\"],[\"ten\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_40_1_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[][0]\",\"result\":[\"one\",\"three\",\"five\",\"seven\",\"nine\",\"ten\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_40_2_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[][1]\",\"result\":[\"two\",\"four\",\"six\",\"eight\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_40_3_foo_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[][0][0]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_40_4_foo_2_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[][2][2]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_40_5_foo_0_0_100() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[][0][0][100]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_41_0_reservations_instances_fo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].foo[].bar\",\"result\":[1,2,4,5,6,8]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_41_1_reservations_instances_fo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].foo[].baz\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_41_2_reservations_instances_no() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].notfoo[].bar\",\"result\":[20,21,22,23,24,25]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_41_3_reservations_instances_no() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].notfoo[].notbar\",\"result\":[[7],[7]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_41_4_reservations_notinstances() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].notinstances[].foo\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_41_5_reservations_instances_fo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].foo[].notbar\",\"result\":[3,[7]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_41_6_reservations_instances_ba() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].bar[].baz\",\"result\":[[1],[2],[3],[4]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_41_7_reservations_instances_ba() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].baz[].baz\",\"result\":[[1,2],[],[],[3,4]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_41_8_reservations_instances_qu() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].qux[].baz\",\"result\":[[],[1,2,3],[4],[],[],[1,2,3],[4],[]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_41_9_reservations_instances_qu() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].qux[].baz[]\",\"result\":[1,2,3,4,1,2,3,4]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_42_0_reservations_instances_fo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].foo\",\"result\":[1,2]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":1},{\"foo\":2}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_42_1_reservations_instances_ba() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].bar\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":1},{\"foo\":2}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_42_2_reservations_notinstances() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].notinstances[].foo\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":1},{\"foo\":2}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_42_3_reservations_notinstances() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].notinstances[].foo\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":1},{\"foo\":2}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_43_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[0]\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_43_1_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[1]\",\"result\":\"two\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_43_2_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[2]\",\"result\":\"three\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_43_3_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[-1]\",\"result\":\"three\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_43_4_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[-2]\",\"result\":\"two\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_43_5_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[-3]\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_44_0_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_44_1_foo_0_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0].bar\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_44_2_foo_1_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[1].bar\",\"result\":\"two\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_44_3_foo_2_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[2].bar\",\"result\":\"three\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_44_4_foo_3_notbar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[3].notbar\",\"result\":\"four\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_44_5_foo_3_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[3].bar\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_44_6_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0]\",\"result\":{\"bar\":\"one\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_44_7_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[1]\",\"result\":{\"bar\":\"two\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_44_8_foo_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[2]\",\"result\":{\"bar\":\"three\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_44_9_foo_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[3]\",\"result\":{\"notbar\":\"four\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_44_10_foo_4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[4]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_45_0_foo_bar_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[0]\",\"result\":\"zero\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_45_1_foo_bar_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[1]\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_45_2_foo_bar_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[2]\",\"result\":\"two\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_45_3_foo_bar_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[3]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_45_4_foo_bar_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[-1]\",\"result\":\"two\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_45_5_foo_bar_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[-2]\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_45_6_foo_bar_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[-3]\",\"result\":\"zero\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_45_7_foo_bar_4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[-4]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_multiselect_46_0_nested_multiselect() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Nested multiselect\",\"expression\":\"[[*]]\",\"result\":[[]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[]").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_46_1_select_on_null() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Select on null\",\"expression\":\"missing.{foo: bar}\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("[]").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_47_0_nested_multiselect() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Nested multiselect\",\"expression\":\"[[*],*]\",\"result\":[null,[\"object\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_48_0_foo_baz_not_there_baz_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[baz[*].not_there || baz[*].bar, qux[0]]\",\"result\":[[\"a\",\"d\"],\"zero\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"baz\":[{\"bar\":\"a\",\"bam\":\"b\",\"boo\":\"c\"},{\"bar\":\"d\",\"bam\":\"e\",\"boo\":\"f\"}],\"qux\":[\"zero\"]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_49_0_foo_baz_bar_boo_qux_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[baz[*].[bar, boo], qux[0]]\",\"result\":[[[\"a\",\"c\"],[\"d\",\"f\"]],\"zero\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"baz\":[{\"bar\":\"a\",\"bam\":\"b\",\"boo\":\"c\"},{\"bar\":\"d\",\"bam\":\"e\",\"boo\":\"f\"}],\"qux\":[\"zero\"]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_50_0_foo_baz_bar_qux_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[baz[*].bar, qux[0]]\",\"result\":[[\"abc\",\"def\"],\"zero\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"baz\":[{\"bar\":\"abc\"},{\"bar\":\"def\"}],\"qux\":[\"zero\"]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_51_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo\",\"result\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_51_1_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[]\",\"result\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_51_2_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar\",\"result\":[[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}],[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_51_3_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar[]\",\"result\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3},{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_51_4_foo_bar_baz_qux() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar[].[baz, qux]\",\"result\":[[1,2],[3,4],[5,6],[7,8]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_51_5_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar[].[baz]\",\"result\":[[1],[3],[5],[7]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_51_6_foo_bar_baz_qux() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar[].[baz, qux][]\",\"result\":[1,2,3,4,5,6,7,8]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"qux\":2,\"baz\":1},{\"qux\":4,\"baz\":3}]},{\"bar\":[{\"qux\":6,\"baz\":5},{\"qux\":8,\"baz\":7}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_52_0_reservations_instances_id() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[*].instances[*].{id: id, name: name}\",\"result\":[[{\"id\":\"id1\",\"name\":\"first\"},{\"id\":\"id2\",\"name\":\"second\"}],[{\"id\":\"id3\",\"name\":\"third\"},{\"id\":\"id4\",\"name\":\"fourth\"}]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"id\":\"id1\",\"name\":\"first\"},{\"id\":\"id2\",\"name\":\"second\"}]},{\"instances\":[{\"id\":\"id3\",\"name\":\"third\"},{\"id\":\"id4\",\"name\":\"fourth\"}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_52_1_reservations_instances_id() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].{id: id, name: name}\",\"result\":[{\"id\":\"id1\",\"name\":\"first\"},{\"id\":\"id2\",\"name\":\"second\"},{\"id\":\"id3\",\"name\":\"third\"},{\"id\":\"id4\",\"name\":\"fourth\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"id\":\"id1\",\"name\":\"first\"},{\"id\":\"id2\",\"name\":\"second\"}]},{\"instances\":[{\"id\":\"id3\",\"name\":\"third\"},{\"id\":\"id4\",\"name\":\"fourth\"}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_52_2_reservations_instances_id() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].[id, name]\",\"result\":[[\"id1\",\"first\"],[\"id2\",\"second\"],[\"id3\",\"third\"],[\"id4\",\"fourth\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"id\":\"id1\",\"name\":\"first\"},{\"id\":\"id2\",\"name\":\"second\"}]},{\"instances\":[{\"id\":\"id3\",\"name\":\"third\"},{\"id\":\"id4\",\"name\":\"fourth\"}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_53_0_foo_bar_bar_baz_1_include() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar: bar.baz[1],includeme: includeme}\",\"result\":{\"bar\":{\"common\":\"second\",\"two\":2},\"includeme\":true}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":[{\"common\":\"first\",\"one\":1},{\"common\":\"second\",\"two\":2}]},\"ignoreme\":1,\"includeme\":true}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_53_1_foo_bar_baz_two_bar_baz_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{\\\"bar.baz.two\\\": bar.baz[1].two, includeme: includeme}\",\"result\":{\"bar.baz.two\":2,\"includeme\":true}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":[{\"common\":\"first\",\"one\":1},{\"common\":\"second\",\"two\":2}]},\"ignoreme\":1,\"includeme\":true}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_53_2_foo_includeme_bar_baz_com() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[includeme, bar.baz[*].common]\",\"result\":[true,[\"first\",\"second\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":[{\"common\":\"first\",\"one\":1},{\"common\":\"second\",\"two\":2}]},\"ignoreme\":1,\"includeme\":true}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_53_3_foo_includeme_bar_baz_non() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[includeme, bar.baz[*].none]\",\"result\":[true,[]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":[{\"common\":\"first\",\"one\":1},{\"common\":\"second\",\"two\":2}]},\"ignoreme\":1,\"includeme\":true}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_53_4_foo_includeme_bar_baz_com() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[includeme, bar.baz[].common]\",\"result\":[true,[\"first\",\"second\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":[{\"common\":\"first\",\"one\":1},{\"common\":\"second\",\"two\":2}]},\"ignoreme\":1,\"includeme\":true}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_54_0_foo_bar_bar_baz_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar: bar, baz: baz}\",\"result\":{\"bar\":1,\"baz\":2}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":2}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_54_1_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,baz]\",\"result\":[1,2]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":2}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_55_0_foo_bar_bar_baz_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar:bar,baz:baz}\",\"result\":{\"bar\":1,\"baz\":[2,3,4]}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":[2,3,4]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_55_1_foo_bar_baz_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,baz[0]]\",\"result\":[1,2]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":[2,3,4]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_55_2_foo_bar_baz_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,baz[1]]\",\"result\":[1,3]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":[2,3,4]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_55_3_foo_bar_baz_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,baz[2]]\",\"result\":[1,4]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":[2,3,4]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_55_4_foo_bar_baz_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,baz[3]]\",\"result\":[1,null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":[2,3,4]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_55_5_foo_bar_0_baz_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar[0],baz[3]]\",\"result\":[null,null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":[2,3,4]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_0_foo_bar_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar: bar}\",\"result\":{\"bar\":\"bar\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_1_foo_bar_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{\\\"bar\\\": bar}\",\"result\":{\"bar\":\"bar\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_2_foo_foo_bar_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{\\\"foo.bar\\\": bar}\",\"result\":{\"foo.bar\":\"bar\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_3_foo_bar_bar_baz_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar: bar, baz: baz}\",\"result\":{\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_4_foo_bar_bar_baz_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{\\\"bar\\\": bar, \\\"baz\\\": baz}\",\"result\":{\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_5_baz_baz_qux_qux() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{\\\"baz\\\": baz, \\\"qux\\\\\\\"\\\": \\\"qux\\\\\\\"\\\"}\",\"result\":{\"baz\":2,\"qux\\\"\":3}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_6_foo_bar_bar_baz_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar:bar,baz:baz}\",\"result\":{\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_7_foo_bar_bar_qux_qux() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar: bar,qux: qux}\",\"result\":{\"bar\":\"bar\",\"qux\":\"qux\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_8_foo_bar_bar_noexist_noexi() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar: bar, noexist: noexist}\",\"result\":{\"bar\":\"bar\",\"noexist\":null}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_9_foo_noexist_noexist_alson() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{noexist: noexist, alsonoexist: alsonoexist}\",\"result\":{\"noexist\":null,\"alsonoexist\":null}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_10_foo_badkey_nokey_nokey_al() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.badkey.{nokey: nokey, alsonokey: alsonokey}\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_11_foo_nested_a_a_b_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.nested.*.{a: a,b: b}\",\"result\":[{\"a\":\"first\",\"b\":\"second\"},{\"a\":\"first\",\"b\":\"second\"},{\"a\":\"first\",\"b\":\"second\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_12_foo_nested_three_a_a_cinn() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.nested.three.{a: a, cinner: c.inner}\",\"result\":{\"a\":\"first\",\"cinner\":\"third\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_13_foo_nested_three_a_a_c_c_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.nested.three.{a: a, c: c.inner.bad.key}\",\"result\":{\"a\":\"first\",\"c\":null}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_14_foo_a_nested_one_a_b_nest() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{a: nested.one.a, b: nested.two.b}\",\"result\":{\"a\":\"first\",\"b\":\"second\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_15_bar_bar_baz_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{bar: bar, baz: baz}\",\"result\":{\"bar\":1,\"baz\":2}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_16_bar_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{bar: bar}\",\"result\":{\"bar\":1}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_17_otherkey_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{otherkey: bar}\",\"result\":{\"otherkey\":1}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_18_no_no_exist_exist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{no: no, exist: exist}\",\"result\":{\"no\":null,\"exist\":null}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_19_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar]\",\"result\":[\"bar\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_20_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,baz]\",\"result\":[\"bar\",\"baz\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_21_foo_bar_qux() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,qux]\",\"result\":[\"bar\",\"qux\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_22_foo_bar_noexist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,noexist]\",\"result\":[\"bar\",null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_56_23_foo_noexist_alsonoexist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[noexist,alsonoexist]\",\"result\":[null,null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"qux\":\"qux\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}}}},\"bar\":1,\"baz\":2,\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_current_57_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"@\",\"result\":{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("current", data).unwrap();
}


#[test]
fn test_current_57_1_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"@.bar\",\"result\":{\"baz\":\"qux\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("current", data).unwrap();
}


#[test]
fn test_current_57_2_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"@.foo[0]\",\"result\":{\"name\":\"a\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}],\"bar\":{\"baz\":\"qux\"}}").unwrap());
    case.assert("current", data).unwrap();
}


#[test]
fn test_filters_58_0_using_in_a_filter_express() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Using @ in a filter expression\",\"expression\":\"foo[?@ < `5`]\",\"result\":[0,1,2,3,4]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_58_1_using_in_a_filter_express() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Using @ in a filter expression\",\"expression\":\"foo[?`5` > @]\",\"result\":[0,1,2,3,4]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_58_2_using_in_a_filter_express() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Using @ in a filter expression\",\"expression\":\"foo[?@ == @]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_59_0_unary_filter_expression() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Unary filter expression\",\"expression\":\"foo[?key]\",\"result\":[{\"key\":true},{\"key\":[0]},{\"key\":{\"a\":\"b\"}},{\"key\":0},{\"key\":1}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":[]},{\"key\":{}},{\"key\":[0]},{\"key\":{\"a\":\"b\"}},{\"key\":0},{\"key\":1},{\"key\":null},{\"notkey\":true}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_59_1_unary_not_filter_expressi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Unary not filter expression\",\"expression\":\"foo[?!key]\",\"result\":[{\"key\":false},{\"key\":[]},{\"key\":{}},{\"key\":null},{\"notkey\":true}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":[]},{\"key\":{}},{\"key\":[0]},{\"key\":{\"a\":\"b\"}},{\"key\":0},{\"key\":1},{\"key\":null},{\"notkey\":true}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_59_2_equality_with_null_rhs() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Equality with null RHS\",\"expression\":\"foo[?key == `null`]\",\"result\":[{\"key\":null},{\"notkey\":true}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":[]},{\"key\":{}},{\"key\":[0]},{\"key\":{\"a\":\"b\"}},{\"key\":0},{\"key\":1},{\"key\":null},{\"notkey\":true}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_60_0_verify_precedence_of_or_a() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Verify precedence of or/and expressions\",\"expression\":\"foo[?a == `1` || b ==`2` && c == `5`]\",\"result\":[{\"a\":1,\"b\":2,\"c\":3}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_60_1_parentheses_can_alter_pre() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Parentheses can alter precedence\",\"expression\":\"foo[?(a == `1` || b ==`2`) && c == `5`]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_60_2_not_expressions_combined_() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Not expressions combined with and/or\",\"expression\":\"foo[?!(a == `1` || b ==`2`)]\",\"result\":[{\"a\":3,\"b\":4}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_61_0_filter_with_or_and_and_ex() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Filter with Or and And expressions\",\"expression\":\"foo[?c == `3` || a == `1` && b == `4`]\",\"result\":[{\"a\":1,\"b\":2,\"c\":3}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_61_1_foo_b_2_a_3_b_4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?b == `2` || a == `3` && b == `4`]\",\"result\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_61_2_foo_a_3_b_4_b_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?a == `3` && b == `4` || b == `2`]\",\"result\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_61_3_foo_a_3_b_4_b_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?(a == `3` && b == `4`) || b == `2`]\",\"result\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_61_4_foo_a_3_b_4_b_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?((a == `3` && b == `4`)) || b == `2`]\",\"result\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_61_5_foo_a_3_b_4_b_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?a == `3` && (b == `4` || b == `2`)]\",\"result\":[{\"a\":3,\"b\":4}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_61_6_foo_a_3_b_4_b_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?a == `3` && ((b == `4` || b == `2`))]\",\"result\":[{\"a\":3,\"b\":4}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_62_0_filter_with_and_expressio() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Filter with and expression\",\"expression\":\"foo[?a == `1` && b == `2`]\",\"result\":[{\"a\":1,\"b\":2}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2},{\"a\":1,\"b\":3}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_62_1_foo_a_1_b_4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?a == `1` && b == `4`]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2},{\"a\":1,\"b\":3}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_63_0_filter_with_or_expression() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Filter with or expression\",\"expression\":\"foo[?name == 'a' || name == 'b']\",\"result\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"},{\"name\":\"c\"}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_63_1_foo_name_a_name_e() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?name == 'a' || name == 'e']\",\"result\":[{\"name\":\"a\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"},{\"name\":\"c\"}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_63_2_foo_name_a_name_b_name_c() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?name == 'a' || name == 'b' || name == 'c']\",\"result\":[{\"name\":\"a\"},{\"name\":\"b\"},{\"name\":\"c\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"},{\"name\":\"c\"}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_64_0_foo_a_1_b_c() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?a==`1`].b.c\",\"result\":[\"x\",\"y\",\"z\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":{\"c\":\"x\"}},{\"a\":1,\"b\":{\"c\":\"y\"}},{\"a\":1,\"b\":{\"c\":\"z\"}},{\"a\":2,\"b\":{\"c\":\"z\"}},{\"a\":1,\"baz\":2}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_65_0_foo_bar_1_bar_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?bar==`1`].bar[0]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"baz\":\"other\",\"foo\":[{\"bar\":1},{\"bar\":2},{\"bar\":3},{\"bar\":4},{\"bar\":1,\"baz\":2}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_66_0_reservations_instances_ba() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[?bar==`1`]\",\"result\":[[{\"foo\":2,\"bar\":1}]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":1,\"bar\":2},{\"foo\":1,\"bar\":3},{\"foo\":1,\"bar\":2},{\"foo\":2,\"bar\":1}]}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_66_1_reservations_instances_ba() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[*].instances[?bar==`1`]\",\"result\":[[{\"foo\":2,\"bar\":1}]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":1,\"bar\":2},{\"foo\":1,\"bar\":3},{\"foo\":1,\"bar\":2},{\"foo\":2,\"bar\":1}]}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_66_2_reservations_instances_ba() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[?bar==`1`][]\",\"result\":[{\"foo\":2,\"bar\":1}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":1,\"bar\":2},{\"foo\":1,\"bar\":3},{\"foo\":1,\"bar\":2},{\"foo\":2,\"bar\":1}]}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_0_foo_key_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `true`]\",\"result\":[{\"key\":true}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_1_foo_key_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `false`]\",\"result\":[{\"key\":false}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_2_foo_key_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `0`]\",\"result\":[{\"key\":0}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_3_foo_key_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `1`]\",\"result\":[{\"key\":1}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_4_foo_key_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `[0]`]\",\"result\":[{\"key\":[0]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_5_foo_key_bar_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `{\\\"bar\\\": [0]}`]\",\"result\":[{\"key\":{\"bar\":[0]}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_6_foo_key_null() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `null`]\",\"result\":[{\"key\":null}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_7_foo_key_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `[1]`]\",\"result\":[{\"key\":[1]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_8_foo_key_a_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `{\\\"a\\\":2}`]\",\"result\":[{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_9_foo_true_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`true` == key]\",\"result\":[{\"key\":true}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_10_foo_false_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`false` == key]\",\"result\":[{\"key\":false}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_11_foo_0_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`0` == key]\",\"result\":[{\"key\":0}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_12_foo_1_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`1` == key]\",\"result\":[{\"key\":1}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_13_foo_0_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`[0]` == key]\",\"result\":[{\"key\":[0]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_14_foo_bar_0_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`{\\\"bar\\\": [0]}` == key]\",\"result\":[{\"key\":{\"bar\":[0]}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_15_foo_null_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`null` == key]\",\"result\":[{\"key\":null}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_16_foo_1_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`[1]` == key]\",\"result\":[{\"key\":[1]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_17_foo_a_2_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`{\\\"a\\\":2}` == key]\",\"result\":[{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_18_foo_key_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key != `true`]\",\"result\":[{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_19_foo_key_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key != `false`]\",\"result\":[{\"key\":true},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_20_foo_key_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key != `0`]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_21_foo_key_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key != `1`]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_22_foo_key_null() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key != `null`]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_23_foo_key_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key != `[1]`]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_24_foo_key_a_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key != `{\\\"a\\\":2}`]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_25_foo_true_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`true` != key]\",\"result\":[{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_26_foo_false_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`false` != key]\",\"result\":[{\"key\":true},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_27_foo_0_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`0` != key]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_28_foo_1_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`1` != key]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_29_foo_null_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`null` != key]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_30_foo_1_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`[1]` != key]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_67_31_foo_a_2_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`{\\\"a\\\":2}` != key]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_68_0_matching_an_expression() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Matching an expression\",\"expression\":\"foo[?top.first == top.last]\",\"result\":[{\"top\":{\"first\":\"foo\",\"last\":\"foo\"}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"top\":{\"first\":\"foo\",\"last\":\"bar\"}},{\"top\":{\"first\":\"foo\",\"last\":\"foo\"}},{\"top\":{\"first\":\"foo\",\"last\":\"baz\"}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_68_1_matching_a_json_array() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Matching a JSON array\",\"expression\":\"foo[?top == `{\\\"first\\\": \\\"foo\\\", \\\"last\\\": \\\"bar\\\"}`]\",\"result\":[{\"top\":{\"first\":\"foo\",\"last\":\"bar\"}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"top\":{\"first\":\"foo\",\"last\":\"bar\"}},{\"top\":{\"first\":\"foo\",\"last\":\"foo\"}},{\"top\":{\"first\":\"foo\",\"last\":\"baz\"}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_69_0_filter_with_subexpression() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Filter with subexpression\",\"expression\":\"foo[?top.name == 'a']\",\"result\":[{\"top\":{\"name\":\"a\"}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"top\":{\"name\":\"a\"}},{\"top\":{\"name\":\"b\"}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_70_0_greater_than_with_a_numbe() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Greater than with a number\",\"expression\":\"foo[?age > `25`]\",\"result\":[{\"age\":30}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_70_1_foo_age_25() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?age >= `25`]\",\"result\":[{\"age\":25},{\"age\":30}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_70_2_greater_than_with_a_numbe() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Greater than with a number\",\"expression\":\"foo[?age > `30`]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_70_3_greater_than_with_a_numbe() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Greater than with a number\",\"expression\":\"foo[?age < `25`]\",\"result\":[{\"age\":20}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_70_4_greater_than_with_a_numbe() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Greater than with a number\",\"expression\":\"foo[?age <= `25`]\",\"result\":[{\"age\":20},{\"age\":25}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_70_5_greater_than_with_a_numbe() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Greater than with a number\",\"expression\":\"foo[?age < `20`]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_70_6_foo_age_20() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?age == `20`]\",\"result\":[{\"age\":20}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_70_7_foo_age_20() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?age != `20`]\",\"result\":[{\"age\":25},{\"age\":30}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_71_0_matching_an_expression() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Matching an expression\",\"expression\":\"foo[?first == last]\",\"result\":[{\"first\":\"foo\",\"last\":\"foo\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"first\":\"foo\",\"last\":\"bar\"},{\"first\":\"foo\",\"last\":\"foo\"},{\"first\":\"foo\",\"last\":\"baz\"}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_71_1_verify_projection_created() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Verify projection created from filter\",\"expression\":\"foo[?first == last].first\",\"result\":[\"foo\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"first\":\"foo\",\"last\":\"bar\"},{\"first\":\"foo\",\"last\":\"foo\"},{\"first\":\"foo\",\"last\":\"baz\"}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_72_0_matching_a_literal() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Matching a literal\",\"expression\":\"*[?[0] == `0`]\",\"result\":[[],[]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1],\"bar\":[2,3]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_73_0_matching_a_literal() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Matching a literal\",\"expression\":\"foo[?name == 'a']\",\"result\":[{\"name\":\"a\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_functions_74_0_map_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"map(&[], array)\",\"result\":[[1,2,3,4],[5,6,7,8,9]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[[1,2,3,[4]],[5,6,7,[8,9]]]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_75_0_map_foo_bar_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"map(&foo.bar, array)\",\"result\":[\"yes1\",\"yes2\",null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[{\"foo\":{\"bar\":\"yes1\"}},{\"foo\":{\"bar\":\"yes2\"}},{\"foo1\":{\"bar\":\"no\"}}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_75_1_map_foo1_bar_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"map(&foo1.bar, array)\",\"result\":[null,null,\"no\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[{\"foo\":{\"bar\":\"yes1\"}},{\"foo\":{\"bar\":\"yes2\"}},{\"foo1\":{\"bar\":\"no\"}}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_75_2_map_foo_bar_baz_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"map(&foo.bar.baz, array)\",\"result\":[null,null,null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[{\"foo\":{\"bar\":\"yes1\"}},{\"foo\":{\"bar\":\"yes2\"}},{\"foo1\":{\"bar\":\"no\"}}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_76_0_map_a_people() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"map(&a, people)\",\"result\":[10,10,10,10,10,10,10,10,10]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"a\":10,\"b\":1,\"c\":\"z\"},{\"a\":10,\"b\":2,\"c\":null},{\"a\":10,\"b\":3},{\"a\":10,\"b\":4,\"c\":\"z\"},{\"a\":10,\"b\":5,\"c\":null},{\"a\":10,\"b\":6},{\"a\":10,\"b\":7,\"c\":\"z\"},{\"a\":10,\"b\":8,\"c\":null},{\"a\":10,\"b\":9}],\"empty\":[]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_76_1_map_c_people() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"map(&c, people)\",\"result\":[\"z\",null,null,\"z\",null,null,\"z\",null,null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"a\":10,\"b\":1,\"c\":\"z\"},{\"a\":10,\"b\":2,\"c\":null},{\"a\":10,\"b\":3},{\"a\":10,\"b\":4,\"c\":\"z\"},{\"a\":10,\"b\":5,\"c\":null},{\"a\":10,\"b\":6},{\"a\":10,\"b\":7,\"c\":\"z\"},{\"a\":10,\"b\":8,\"c\":null},{\"a\":10,\"b\":9}],\"empty\":[]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_76_2_map_a_badkey() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"map(&a, badkey)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"a\":10,\"b\":1,\"c\":\"z\"},{\"a\":10,\"b\":2,\"c\":null},{\"a\":10,\"b\":3},{\"a\":10,\"b\":4,\"c\":\"z\"},{\"a\":10,\"b\":5,\"c\":null},{\"a\":10,\"b\":6},{\"a\":10,\"b\":7,\"c\":\"z\"},{\"a\":10,\"b\":8,\"c\":null},{\"a\":10,\"b\":9}],\"empty\":[]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_76_3_map_foo_empty() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"map(&foo, empty)\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"a\":10,\"b\":1,\"c\":\"z\"},{\"a\":10,\"b\":2,\"c\":null},{\"a\":10,\"b\":3},{\"a\":10,\"b\":4,\"c\":\"z\"},{\"a\":10,\"b\":5,\"c\":null},{\"a\":10,\"b\":6},{\"a\":10,\"b\":7,\"c\":\"z\"},{\"a\":10,\"b\":8,\"c\":null},{\"a\":10,\"b\":9}],\"empty\":[]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_77_0_stable_sort_order() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"stable sort order\",\"expression\":\"sort_by(people, &age)\",\"result\":[{\"age\":10,\"order\":\"1\"},{\"age\":10,\"order\":\"2\"},{\"age\":10,\"order\":\"3\"},{\"age\":10,\"order\":\"4\"},{\"age\":10,\"order\":\"5\"},{\"age\":10,\"order\":\"6\"},{\"age\":10,\"order\":\"7\"},{\"age\":10,\"order\":\"8\"},{\"age\":10,\"order\":\"9\"},{\"age\":10,\"order\":\"10\"},{\"age\":10,\"order\":\"11\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":10,\"order\":\"1\"},{\"age\":10,\"order\":\"2\"},{\"age\":10,\"order\":\"3\"},{\"age\":10,\"order\":\"4\"},{\"age\":10,\"order\":\"5\"},{\"age\":10,\"order\":\"6\"},{\"age\":10,\"order\":\"7\"},{\"age\":10,\"order\":\"8\"},{\"age\":10,\"order\":\"9\"},{\"age\":10,\"order\":\"10\"},{\"age\":10,\"order\":\"11\"}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_0_sort_by_field_expression() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"sort by field expression\",\"expression\":\"sort_by(people, &age)\",\"result\":[{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3},{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_1_sort_by_people_age_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort_by(people, &age_str)\",\"result\":[{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3},{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_2_sort_by_function_expressi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"sort by function expression\",\"expression\":\"sort_by(people, &to_number(age_str))\",\"result\":[{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3},{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_3_function_projection_on_so() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"function projection on sort_by function\",\"expression\":\"sort_by(people, &age)[].name\",\"result\":[3,\"a\",\"c\",\"b\",\"d\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_4_sort_by_people_extra() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort_by(people, &extra)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_5_sort_by_people_bool() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort_by(people, &bool)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_6_sort_by_people_name() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort_by(people, &name)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_7_sort_by_people_name() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort_by(people, name)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_8_sort_by_people_age_extra() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort_by(people, &age)[].extra\",\"result\":[\"foo\",\"bar\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_9_sort_by_age() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort_by(`[]`, &age)\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_10_max_by_people_age() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max_by(people, &age)\",\"result\":{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_11_max_by_people_age_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max_by(people, &age_str)\",\"result\":{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_12_max_by_people_bool() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max_by(people, &bool)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_13_max_by_people_extra() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max_by(people, &extra)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_14_max_by_people_to_number_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max_by(people, &to_number(age_str))\",\"result\":{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_15_min_by_people_age() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min_by(people, &age)\",\"result\":{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_16_min_by_people_age_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min_by(people, &age_str)\",\"result\":{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_17_min_by_people_bool() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min_by(people, &bool)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_18_min_by_people_extra() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min_by(people, &extra)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_78_19_min_by_people_to_number_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min_by(people, &to_number(age_str))\",\"result\":{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"name\":\"a\",\"extra\":\"foo\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"name\":\"b\",\"extra\":\"bar\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_79_0_function_projection_on_va() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"function projection on variadic function\",\"expression\":\"foo[].not_null(f, e, d, c, b, a)\",\"result\":[\"b\",\"c\",\"d\",\"e\",\"f\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"b\":\"b\",\"a\":\"a\"},{\"c\":\"c\",\"b\":\"b\"},{\"d\":\"d\",\"c\":\"c\"},{\"e\":\"e\",\"d\":\"d\"},{\"f\":\"f\",\"e\":\"e\"}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_0_abs_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs(foo)\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_1_abs_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs(foo)\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_2_abs_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs(str)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_3_abs_array_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs(array[1])\",\"result\":3}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_4_abs_array_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs(array[1])\",\"result\":3}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_5_abs_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs(`false`)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_6_abs_24() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs(`-24`)\",\"result\":24}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_7_abs_24() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs(`-24`)\",\"result\":24}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_8_abs_1_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs(`1`, `2`)\",\"error\":\"invalid-arity\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_9_abs() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs()\",\"error\":\"invalid-arity\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_10_unknown_function_1_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"unknown_function(`1`, `2`)\",\"error\":\"unknown-function\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_11_avg_numbers() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"avg(numbers)\",\"result\":2.75}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_12_avg_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"avg(array)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_13_avg_abc() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"avg('abc')\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_14_avg_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"avg(foo)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_15_avg() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"avg(@)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_16_avg_strings() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"avg(strings)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_17_ceil_1_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ceil(`1.2`)\",\"result\":2}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_18_ceil_decimals_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ceil(decimals[0])\",\"result\":2}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_19_ceil_decimals_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ceil(decimals[1])\",\"result\":2}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_20_ceil_decimals_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ceil(decimals[2])\",\"result\":-1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_21_ceil_string() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ceil('string')\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_22_contains_abc_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"contains('abc', 'a')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_23_contains_abc_d() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"contains('abc', 'd')\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_24_contains_false_d() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"contains(`false`, 'd')\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_25_contains_strings_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"contains(strings, 'a')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_26_contains_decimals_1_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"contains(decimals, `1.2`)\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_27_contains_decimals_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"contains(decimals, `false`)\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_28_ends_with_str_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ends_with(str, 'r')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_29_ends_with_str_tr() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ends_with(str, 'tr')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_30_ends_with_str_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ends_with(str, 'Str')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_31_ends_with_str_sstr() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ends_with(str, 'SStr')\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_32_ends_with_str_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ends_with(str, 'foo')\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_33_ends_with_str_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ends_with(str, `0`)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_34_floor_1_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"floor(`1.2`)\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_35_floor_string() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"floor('string')\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_36_floor_decimals_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"floor(decimals[0])\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_37_floor_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"floor(foo)\",\"result\":-1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_38_floor_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"floor(str)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_39_length_abc() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length('abc')\",\"result\":3}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_40_length_okfoo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length('✓foo')\",\"result\":4}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_41_length() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length('')\",\"result\":0}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_42_length() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length(@)\",\"result\":12}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_43_length_strings_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length(strings[0])\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_44_length_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length(str)\",\"result\":3}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_45_length_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length(array)\",\"result\":6}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_46_length_objects() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length(objects)\",\"result\":2}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_47_length_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length(`false`)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_48_length_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length(foo)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_49_length_strings_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length(strings[0])\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_50_max_numbers() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max(numbers)\",\"result\":5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_51_max_decimals() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max(decimals)\",\"result\":1.2}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_52_max_strings() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max(strings)\",\"result\":\"c\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_53_max_abc() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max(abc)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_54_max_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max(array)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_55_max_decimals() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max(decimals)\",\"result\":1.2}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_56_max_empty_list() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max(empty_list)\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_57_merge() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"merge(`{}`)\",\"result\":{}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_58_merge() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"merge(`{}`, `{}`)\",\"result\":{}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_59_merge_a_1_b_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"merge(`{\\\"a\\\": 1}`, `{\\\"b\\\": 2}`)\",\"result\":{\"a\":1,\"b\":2}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_60_merge_a_1_a_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"merge(`{\\\"a\\\": 1}`, `{\\\"a\\\": 2}`)\",\"result\":{\"a\":2}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_61_merge_a_1_b_2_a_2_c_3_d_4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"merge(`{\\\"a\\\": 1, \\\"b\\\": 2}`, `{\\\"a\\\": 2, \\\"c\\\": 3}`, `{\\\"d\\\": 4}`)\",\"result\":{\"a\":2,\"b\":2,\"c\":3,\"d\":4}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_62_min_numbers() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min(numbers)\",\"result\":-1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_63_min_decimals() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min(decimals)\",\"result\":-1.5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_64_min_abc() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min(abc)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_65_min_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min(array)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_66_min_empty_list() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min(empty_list)\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_67_min_decimals() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min(decimals)\",\"result\":-1.5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_68_min_strings() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min(strings)\",\"result\":\"a\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_69_type_abc() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type('abc')\",\"result\":\"string\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_70_type_1_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(`1.0`)\",\"result\":\"number\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_71_type_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(`2`)\",\"result\":\"number\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_72_type_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(`true`)\",\"result\":\"boolean\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_73_type_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(`false`)\",\"result\":\"boolean\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_74_type_null() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(`null`)\",\"result\":\"null\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_75_type_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(`[0]`)\",\"result\":\"array\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_76_type_a_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(`{\\\"a\\\": \\\"b\\\"}`)\",\"result\":\"object\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_77_type() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(@)\",\"result\":\"object\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_78_sort_keys_objects() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort(keys(objects))\",\"result\":[\"bar\",\"foo\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_79_keys_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"keys(foo)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_80_keys_strings() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"keys(strings)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_81_keys_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"keys(`false`)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_82_sort_values_objects() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort(values(objects))\",\"result\":[\"bar\",\"baz\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_83_keys_empty_hash() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"keys(empty_hash)\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_84_values_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"values(foo)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_85_join_strings() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join(', ', strings)\",\"result\":\"a, b, c\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_86_join_strings() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join(', ', strings)\",\"result\":\"a, b, c\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_87_join_a_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join(',', `[\\\"a\\\", \\\"b\\\"]`)\",\"result\":\"a,b\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_88_join_a_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join(',', `[\\\"a\\\", 0]`)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_89_join_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join(', ', str)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_90_join_strings() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join('|', strings)\",\"result\":\"a|b|c\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_91_join_2_strings() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join(`2`, strings)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_92_join_decimals() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join('|', decimals)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_93_join_decimals_to_string() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join('|', decimals[].to_string(@))\",\"result\":\"1.01|1.2|-1.5\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_94_join_empty_list() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join('|', empty_list)\",\"result\":\"\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_95_reverse_numbers() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reverse(numbers)\",\"result\":[5,4,3,-1]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_96_reverse_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reverse(array)\",\"result\":[\"100\",\"a\",5,4,3,-1]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_97_reverse() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reverse(`[]`)\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_98_reverse() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reverse('')\",\"result\":\"\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_99_reverse_hello_world() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reverse('hello world')\",\"result\":\"dlrow olleh\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_100_starts_with_str_s() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"starts_with(str, 'S')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_101_starts_with_str_st() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"starts_with(str, 'St')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_102_starts_with_str_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"starts_with(str, 'Str')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_103_starts_with_str_string() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"starts_with(str, 'String')\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_104_starts_with_str_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"starts_with(str, `0`)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_105_sum_numbers() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sum(numbers)\",\"result\":11}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_106_sum_decimals() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sum(decimals)\",\"result\":0.71}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_107_sum_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sum(array)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_108_sum_array_to_number() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sum(array[].to_number(@))\",\"result\":111}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_109_sum() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sum(`[]`)\",\"result\":0}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_110_to_array_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_array('foo')\",\"result\":[\"foo\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_111_to_array_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_array(`0`)\",\"result\":[0]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_112_to_array_objects() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_array(objects)\",\"result\":[{\"foo\":\"bar\",\"bar\":\"baz\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_113_to_array_1_2_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_array(`[1, 2, 3]`)\",\"result\":[1,2,3]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_114_to_array_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_array(false)\",\"result\":[false]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_115_to_string_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_string('foo')\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_116_to_string_1_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_string(`1.2`)\",\"result\":\"1.2\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_117_to_string_0_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_string(`[0, 1]`)\",\"result\":\"[0,1]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_118_to_number_1_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number('1.0')\",\"result\":1.0}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_119_to_number_1_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number('1.1')\",\"result\":1.1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_120_to_number_4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number('4')\",\"result\":4}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_121_to_number_notanumber() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number('notanumber')\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_122_to_number_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number(`false`)\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_123_to_number_null() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number(`null`)\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_124_to_number_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number(`[0]`)\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_125_to_number_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number(`{\\\"foo\\\": 0}`)\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_126_to_string_1_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"to_string\\\"(`1.0`)\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_127_sort_numbers() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort(numbers)\",\"result\":[-1,3,4,5]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_128_sort_strings() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort(strings)\",\"result\":[\"a\",\"b\",\"c\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_129_sort_decimals() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort(decimals)\",\"result\":[-1.5,1.01,1.2]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_130_sort_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort(array)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_131_sort_abc() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort(abc)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_132_sort_empty_list() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort(empty_list)\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_133_sort() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort(@)\",\"error\":\"invalid-type\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_134_not_null_unknown_key_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"not_null(unknown_key, str)\",\"result\":\"Str\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_135_not_null_unknown_key_foo_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"not_null(unknown_key, foo.bar, empty_list, str)\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_136_not_null_unknown_key_null() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"not_null(unknown_key, null_key, empty_list, str)\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_137_not_null_all_expressions_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"not_null(all, expressions, are_null)\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_138_not_null() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"not_null()\",\"error\":\"invalid-arity\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_139_function_projection_on_si() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"function projection on single arg function\",\"expression\":\"numbers[].to_string(@)\",\"result\":[\"-1\",\"3\",\"4\",\"5\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_80_140_function_projection_on_si() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"function projection on single arg function\",\"expression\":\"array[].to_number(@)\",\"result\":[-1,3,4,5,100]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":-1,\"zero\":0,\"numbers\":[-1,3,4,5],\"array\":[-1,3,4,5,\"a\",\"100\"],\"strings\":[\"a\",\"b\",\"c\"],\"decimals\":[1.01,1.2,-1.5],\"str\":\"Str\",\"false\":false,\"empty_list\":[],\"empty_hash\":{},\"objects\":{\"foo\":\"bar\",\"bar\":\"baz\"},\"null_key\":null}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_syntax_81_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*||*|*|*\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("[]").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_81_1_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*[]||[*]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[]").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_81_2_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[*.*]\",\"result\":[null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[]").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_82_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_82_1_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"foo\\\"\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_82_2_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\\\\\"\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_82_3_u() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\u\\\"\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_83_0_bar_anything() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bar.`\\\"anything\\\"`\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_83_1_bar_baz_noexists_literal() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bar.baz.noexists.`\\\"literal\\\"`\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_83_2_literal_wildcard_projecti() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Literal wildcard projection\",\"expression\":\"foo[*].`\\\"literal\\\"`\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_83_3_foo_name_literal() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].name.`\\\"literal\\\"`\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_83_4_foo_name_literal() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].name.`\\\"literal\\\"`\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_83_5_foo_name_literal_subliter() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].name.`\\\"literal\\\"`.`\\\"subliteral\\\"`\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_83_6_projecting_a_literal_onto() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Projecting a literal onto an empty list\",\"expression\":\"foo[*].name.noexist.`\\\"literal\\\"`\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_83_7_foo_name_noexist_literal() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].name.noexist.`\\\"literal\\\"`\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_83_8_twolen_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"twolen[*].`\\\"foo\\\"`\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_83_9_two_level_projection_of_a() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Two level projection of a literal\",\"expression\":\"twolen[*].threelen[*].`\\\"bar\\\"`\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_83_10_two_level_flattened_proje() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Two level flattened projection of a literal\",\"expression\":\"twolen[].threelen[].`\\\"bar\\\"`\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_83_11_expects_closing() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"expects closing ]\",\"expression\":\"foo[? @ | @\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_0_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?bar==`\\\"baz\\\"`]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_1_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[? bar == `\\\"baz\\\"` ]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_2_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[ ?bar==`\\\"baz\\\"`]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_3_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?bar==]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_4_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?==]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_5_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?==bar]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_6_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?bar==baz?]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_7_foo_a_b_c_d_e_f() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?a.b.c==d.e.f]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_8_foo_bar_0_1_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?bar==`[0, 1, 2]`]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_9_foo_bar_a_b_c() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?bar==`[\\\"a\\\", \\\"b\\\", \\\"c\\\"]`]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_10_literal_char_not_escaped() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Literal char not escaped\",\"expression\":\"foo[?bar==`[\\\"foo`bar\\\"]`]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_11_literal_char_escaped() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Literal char escaped\",\"expression\":\"foo[?bar==`[\\\"foo\\\\`bar\\\"]`]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_12_unknown_comparator() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Unknown comparator\",\"expression\":\"foo[?bar<>baz]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_13_unknown_comparator() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Unknown comparator\",\"expression\":\"foo[?bar^baz]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_14_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[bar==baz]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_15_quoted_identifier_in_filt() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Quoted identifier in filter expression no spaces\",\"expression\":\"[?\\\"\\\\\\\\\\\">`\\\"foo\\\"`]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_84_16_quoted_identifier_in_filt() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Quoted identifier in filter expression with spaces\",\"expression\":\"[?\\\"\\\\\\\\\\\" > `\\\"foo\\\"`]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_85_0_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo || bar\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_85_1_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo ||\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_85_2_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.|| bar\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_85_3_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\" || foo\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_85_4_foo_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo || || foo\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_85_5_foo_a_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[a || b]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_85_6_foo_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[a ||]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_85_7_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"foo\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_0_no_key_or_value() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"No key or value\",\"expression\":\"a{}\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_1_no_closing_token() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"No closing token\",\"expression\":\"a{\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_2_not_a_key_value_pair() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Not a key value pair\",\"expression\":\"a{foo}\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_3_missing_value_and_closing() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing value and closing character\",\"expression\":\"a{foo:\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_4_missing_closing_character() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing closing character\",\"expression\":\"a{foo: 0\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_5_missing_value() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing value\",\"expression\":\"a{foo:}\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_6_trailing_comma_and_no_clo() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Trailing comma and no closing character\",\"expression\":\"a{foo: 0, \",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_7_missing_value_with_traili() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing value with trailing comma\",\"expression\":\"a{foo: ,}\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_8_accessing_array_using_an_() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Accessing Array using an identifier\",\"expression\":\"a{foo: bar}\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_9_a_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"a{foo: 0}\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_10_missing_key_value_pair() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing key-value pair\",\"expression\":\"a.{}\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_11_not_a_key_value_pair() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Not a key-value pair\",\"expression\":\"a.{foo}\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_12_valid_multi_select_hash_e() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Valid multi-select hash extraction\",\"expression\":\"a.{foo: bar}\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_13_valid_multi_select_hash_e() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Valid multi-select hash extraction\",\"expression\":\"a.{foo: bar, baz: bam}\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_14_trailing_comma() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Trailing comma\",\"expression\":\"a.{foo: bar, }\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_15_missing_key_in_second_key() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing key in second key-value pair\",\"expression\":\"a.{foo: bar, baz}\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_16_missing_value_in_second_k() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing value in second key-value pair\",\"expression\":\"a.{foo: bar, baz:}\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_17_trailing_comma() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Trailing comma\",\"expression\":\"a.{foo: bar, baz: bam, }\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_18_nested_multi_select() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Nested multi select\",\"expression\":\"{\\\"\\\\\\\\\\\":{\\\" \\\":*}}\",\"result\":{\"\\\\\":{\" \":[\"object\"]}}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_86_19_missing_closing_after_a_v() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing closing } after a valid nud\",\"expression\":\"{a: @\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_0_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_1_valid_multi_select_of_a_l() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Valid multi-select of a list\",\"expression\":\"foo[0, 1]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_2_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[0]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_3_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_4_multi_select_of_a_list_wi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list with trailing comma\",\"expression\":\"foo[0, ]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_5_multi_select_of_a_list_wi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list with trailing comma and no close\",\"expression\":\"foo[0,\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_6_multi_select_of_a_list_wi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list with trailing comma and no close\",\"expression\":\"foo.[a\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_7_multi_select_of_a_list_wi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list with extra comma\",\"expression\":\"foo[0,, 1]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_8_multi_select_of_a_list_us() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list using an identifier index\",\"expression\":\"foo[abc]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_9_multi_select_of_a_list_us() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list using identifier indices\",\"expression\":\"foo[abc, def]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_10_multi_select_of_a_list_us() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list using an identifier index\",\"expression\":\"foo[abc, 1]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_11_multi_select_of_a_list_us() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list using an identifier index with trailing comma\",\"expression\":\"foo[abc, ]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_12_valid_multi_select_of_a_h() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Valid multi-select of a hash using an identifier index\",\"expression\":\"foo.[abc]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_13_valid_multi_select_of_a_h() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Valid multi-select of a hash\",\"expression\":\"foo.[abc, def]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_14_multi_select_of_a_hash_us() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a hash using a numeric index\",\"expression\":\"foo.[abc, 1]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_15_multi_select_of_a_hash_wi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a hash with a trailing comma\",\"expression\":\"foo.[abc, ]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_16_multi_select_of_a_hash_wi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a hash with extra commas\",\"expression\":\"foo.[abc,, def]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_87_17_multi_select_of_a_hash_us() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a hash using number indices\",\"expression\":\"foo.[0, 1]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_88_0_slice_expected_colon_or_r() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"slice expected colon or rbracket\",\"expression\":\"[:@]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_88_1_slice_has_too_many_colons() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"slice has too many colons\",\"expression\":\"[:::]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_88_2_slice_expected_number() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"slice expected number\",\"expression\":\"[:@:]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_88_3_slice_expected_number_of_() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"slice expected number of colon\",\"expression\":\"[:1@]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_89_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_89_1_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_89_2_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.[0]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_89_3_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.[\\\"0\\\"]\",\"result\":[[null]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_89_4_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[*].bar\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_89_5_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[*][0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_89_6_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[#]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_89_7_missing_rbracket_for_led_() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"missing rbracket for led wildcard index\",\"expression\":\"led[*\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_90_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_91_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*\",\"result\":[\"object\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_91_1_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.*\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_91_2_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.foo\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_91_3_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*[0]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_91_4_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\".*\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_91_5_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*foo\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_91_6_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*0\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_91_7_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*]bar\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_91_8_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*]*\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_92_0_invalid_start_of_function() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"invalid start of function\",\"expression\":\"@(foo)\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_92_1_function_names_cannot_be_() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"function names cannot be quoted\",\"expression\":\"\\\"foo\\\"(bar)\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_93_0_missing_closing_paren() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"missing closing paren\",\"expression\":\"(@\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_94_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"![!(!\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_95_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\".\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_95_1_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\":\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_95_2_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\",\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_95_3_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_95_4_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_95_5_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"}\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_95_6_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_95_7_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\")\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_95_8_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"(\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_95_9_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"((&\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_95_10_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"a[\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_95_11_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"a]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_95_12_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"a][\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_95_13_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_96_0_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_96_1_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.1\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_96_2_foo_11() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.-11\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_96_3_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_96_4_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\".foo\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_96_5_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo..bar\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_96_6_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar.\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_96_7_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[.]\",\"error\":\"syntax\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_identifiers_97_0_ud834_udd1e() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\uD834\\\\uDD1E\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"𝄞\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_98_0_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"<\\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"<\\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_99_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_100_0_c() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_C\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_C\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_101_0_vh2_h() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"VH2&H\\\\\\\\\\\\/\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"VH2&H\\\\/\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_102_0_su() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sU\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"sU\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_103_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"?\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"?\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_104_0_5() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"5\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"5\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_105_0_xiuo9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"xIUo9\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"xIUo9\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_106_0_b7eo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"b7eo\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"b7eo\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_107_0_8() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\\8\\\\\\\\\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\\8\\\\\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_108_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"0\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"0\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_109_0_7() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_7\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_7\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_110_0_6() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"6\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"6\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_111_0_b_n() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"!\\\\b\\\\n\u{d1a52}\\\\\\\"\\\\\\\"\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"!\\b\\n\u{d1a52}\\\"\\\"\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_112_0_m_k() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"M_k\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"M_k\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_113_0_9_r_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"9\\\\r\\\\\\\\R\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"9\\r\\\\R\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_114_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"&\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"&\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_115_0_hh() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Hh\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Hh\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_116_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\"!\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"!\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_117_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"!,\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"!,\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_118_0_f() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_F\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_F\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_119_0_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\b%\\\\\\\"\u{9e10f}\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\b%\\\"\u{9e10f}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_120_0_z9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Z9\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Z9\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_121_0_tx_uabbb() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\tX$\\\\uABBb\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\tX$ꮻ\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_122_0_bq() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"BQ\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"BQ\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_123_0_w_a0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"W_a0_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"W_a0_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_124_0_i() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"I_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"I_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_125_0_n_f() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\n\\\\\\\\\\\\f\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\n\\\\\\f\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_126_0_tk_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\tK\\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\tK\\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_127_0_62l() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_62L\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_62L\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_128_0_d7() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"D7\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"D7\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_129_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\"\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_130_0_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\b+\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\b+\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_131_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_0\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_0\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_132_0_yu_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"YU_2\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"YU_2\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_133_0_z_m() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"z_M_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"z_M_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_134_0_z_5() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Z_5\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Z_5\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_135_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\u{f5141}\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\u{f5141}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_136_0_434() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"__434\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"__434\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_137_0_zs1dc() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"zs1DC\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"zs1DC\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_138_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\u{103c02}\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\u{103c02}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_139_0_bw_6hg_gl() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_BW_6Hg_Gl\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_BW_6Hg_Gl\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_140_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\/\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"/\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_141_0_f() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\":\\\\f\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\":\\f\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_142_0_uefac() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\u{c77c7}\\\\\\\\ueFAc\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\u{c77c7}\\\\ueFAc\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_143_0_t_n_b_z() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\/+\\\\t\\\\n\\\\b!Z\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"/+\\t\\n\\b!Z\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_144_0_b_q() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"B_q\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"B_q\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_145_0_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\",\\\\t;\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\",\\t;\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_146_0_bp() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Bp\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Bp\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_147_0_ns_n() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\nS \\\\n\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\nS \\n\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_148_0_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"B__\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"B__\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_149_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"#\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"#\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_150_0_t_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\t&\\\\\\\\\\\\r\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\t&\\\\\\r\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_151_0_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_152_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"<\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"<\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_153_0_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\b\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\b\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_154_0_gy() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Gy\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Gy\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_155_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"/\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"/\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_156_0_fa0_9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"fa0_9\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"fa0_9\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_157_0_u_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"U)\\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"U)\\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_158_0_ueebf() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\uEEbF\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\u{eebf}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_159_0_i_n() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"!I\\\\n\\\\/\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"!I\\n/\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_160_0_hu() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"hU\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hU\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_161_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"; !\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"; !\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_162_0_hvu() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"hvu\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hvu\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_163_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\">\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\">\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_164_0_b_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\b\\\\b\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\b\\b\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_165_0_kl() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Kl\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Kl\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_166_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\"\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_167_0_s() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\\\u{de8a4}S\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\\\u{de8a4}S\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_168_0_7() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"7\\\\\\\"\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"7\\\"\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_169_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\"!\\\\/\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"!/\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_170_0_mg() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Mg\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Mg\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_171_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"+\\\\\\\"\\\\\\\"\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"+\\\"\\\"\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_172_0_r_fb() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\r\\\\fB \\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\r\\fB \":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_173_0_m() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"m_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"m_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_174_0_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\r\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\r\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_175_0_u4fdc() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\\\\\\u4FDc\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\\俜\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_176_0_f() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\f\u{e5333}\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\f\u{e5333}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_177_0_n() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\n\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\n\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_178_0_obf() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Obf\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Obf\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_179_0_rb() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\rB\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\rB\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_180_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\":\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\":\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_181_0_j() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_j\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_j\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_182_0_r_8() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_r_8\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_r_8\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_183_0_o() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"O_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"O_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_184_0_b_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\b\\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\b\\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_185_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"__\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"__\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_186_0_p9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"p9\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"p9\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_187_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"-\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"-\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_188_0_r7() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"r7\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"r7\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_189_0_r_f() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\r\\\\f:\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\r\\f:\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_190_0_rr9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"RR9_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"RR9_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_191_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\\\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\\\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_192_0_q_7gl8() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_Q__7GL8\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_Q__7GL8\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_193_0_q() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Q\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Q\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_194_0_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"r\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"r\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_195_0_b_ud8cb_udc83() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\\\\\\b\\\\ud8cb\\\\udc83\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\\\\b\u{42c83}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_196_0_9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"9\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"9\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_197_0_sna() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sNA_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"sNA_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_198_0_ubbce_ufafb() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\ubBcE\\\\ufAfB\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"믎\u{fafb}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_199_0_u_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"<<U\\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"<<U\\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_200_0_tl7() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"tL7\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"tL7\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_201_0_uaba1_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\uaBA1\\\\r\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"ꮡ\\r\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_202_0_6w() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_6W\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_6W\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_203_0_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"R!\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"R!\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_204_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\" [\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\" [\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_205_0_tm() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"tM\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"tM\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_206_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"!\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"!\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_207_0_e4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"E4\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"E4\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_208_0_f() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\f\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\f\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_209_0_h() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"H\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"H\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_210_0_v24_w() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"v24_W\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"v24_W\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_211_0_t4_ud9da_udd15() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\t4\\\\ud9da\\\\udd15\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\t4\u{86915}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_212_0_x() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_X\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_X\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_213_0_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_214_0_v2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"v2\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"v2\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_215_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\" \\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\" \":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_216_0_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\" \\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\" \\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_217_0_tf_ucebb() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\tF\\\\uCebb\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\tF캻\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_218_0_x() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"x\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"x\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_219_0_y_1623() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Y_1623\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Y_1623\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_220_0_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"!\\\\r\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"!\\r\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_221_0_l() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"__L\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"__L\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_basic_222_0_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.\\\"1\\\"\",\"result\":[\"one\",\"two\",\"three\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"1\":[\"one\",\"two\",\"three\"],\"-1\":\"bar\"}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_222_1_foo_1_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.\\\"1\\\"[0]\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"1\":[\"one\",\"two\",\"three\"],\"-1\":\"bar\"}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_222_2_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.\\\"-1\\\"\",\"result\":\"bar\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"1\":[\"one\",\"two\",\"three\"],\"-1\":\"bar\"}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_223_0_one() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_223_1_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"two\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_223_2_three() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"three\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_223_3_one_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one.two\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_224_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo\",\"result\":{\"bar\":[\"one\",\"two\",\"three\"]}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"one\",\"two\",\"three\"]}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_224_1_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar\",\"result\":[\"one\",\"two\",\"three\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"one\",\"two\",\"three\"]}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_225_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo\",\"result\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_225_1_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar\",\"result\":{\"baz\":\"correct\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_225_2_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar.baz\",\"result\":\"correct\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_225_3_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo\\n.\\nbar\\n.baz\",\"result\":\"correct\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_225_4_foo_bar_baz_bad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar.baz.bad\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_225_5_foo_bar_bad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar.bad\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_225_6_foo_bad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bad\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_225_7_bad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bad\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_225_8_bad_morebad_morebad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bad.morebad.morebad\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}

