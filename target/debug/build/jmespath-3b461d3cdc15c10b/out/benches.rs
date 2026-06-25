

fn benchmarks_0_0_field_50_parse_lex(b: &mut Bencher) {
    b.iter(|| { parse("j49.j48.j47.j46.j45.j44.j43.j42.j41.j40.j39.j38.j37.j36.j35.j34.j33.j32.j31.j30.j29.j28.j27.j26.j25.j24.j23.j22.j21.j20.j19.j18.j17.j16.j15.j14.j13.j12.j11.j10.j9.j8.j7.j6.j5.j4.j3.j2.j1.j0").ok() });
}



fn benchmarks_0_1_pipe_50_parse_lex(b: &mut Bencher) {
    b.iter(|| { parse("j49|j48|j47|j46|j45|j44|j43|j42|j41|j40|j39|j38|j37|j36|j35|j34|j33|j32|j31|j30|j29|j28|j27|j26|j25|j24|j23|j22|j21|j20|j19|j18|j17|j16|j15|j14|j13|j12|j11|j10|j9|j8|j7|j6|j5|j4|j3|j2|j1|j0").ok() });
}



fn benchmarks_0_2_index_50_parse_lex(b: &mut Bencher) {
    b.iter(|| { parse("[49][48][47][46][45][44][43][42][41][40][39][38][37][36][35][34][33][32][31][30][29][28][27][26][25][24][23][22][21][20][19][18][17][16][15][14][13][12][11][10][9][8][7][6][5][4][3][2][1][0]").ok() });
}



fn benchmarks_0_3_long_raw_string_literal_parse_lex(b: &mut Bencher) {
    b.iter(|| { parse("'abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz'").ok() });
}



fn benchmarks_0_4_deep_projection_104_parse_lex(b: &mut Bencher) {
    b.iter(|| { parse("a[*].b[*].c[*].d[*].e[*].f[*].g[*].h[*].i[*].j[*].k[*].l[*].m[*].n[*].o[*].p[*].q[*].r[*].s[*].t[*].u[*].v[*].w[*].x[*].y[*].z[*].a[*].b[*].c[*].d[*].e[*].f[*].g[*].h[*].i[*].j[*].k[*].l[*].m[*].n[*].o[*].p[*].q[*].r[*].s[*].t[*].u[*].v[*].w[*].x[*].y[*].z[*].a[*].b[*].c[*].d[*].e[*].f[*].g[*].h[*].i[*].j[*].k[*].l[*].m[*].n[*].o[*].p[*].q[*].r[*].s[*].t[*].u[*].v[*].w[*].x[*].y[*].z[*].a[*].b[*].c[*].d[*].e[*].f[*].g[*].h[*].i[*].j[*].k[*].l[*].m[*].n[*].o[*].p[*].q[*].r[*].s[*].t[*].u[*].v[*].w[*].x[*].y[*].z[*]").ok() });
}



fn benchmarks_0_5_filter_projection_parse_lex(b: &mut Bencher) {
    b.iter(|| { parse("foo[bar > baz][qux > baz]").ok() });
}



fn benchmarks_1_0_deep_ands_parse_lex(b: &mut Bencher) {
    b.iter(|| { parse("a && b && c && d && e && f && g && h && i && j && k && l && m && n && o && p && q && r && s && t && u && v && w && x && y && z").ok() });
}



fn benchmarks_1_0_deep_ands_interpret(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"a\":0,\"b\":1,\"c\":2,\"d\":3,\"e\":4,\"f\":5,\"g\":6,\"h\":7,\"i\":8,\"j\":9,\"k\":10,\"l\":11,\"m\":12,\"n\":13,\"o\":14,\"p\":15,\"q\":16,\"r\":17,\"s\":18,\"t\":19,\"u\":20,\"v\":21,\"w\":22,\"x\":23,\"y\":24,\"z\":25}").expect("Invalid JSON given"));
    let expr = compile("a && b && c && d && e && f && g && h && i && j && k && l && m && n && o && p && q && r && s && t && u && v && w && x && y && z").unwrap();
    b.iter(|| { expr.search(&data).ok() });
}



fn benchmarks_1_0_deep_ands_full(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"a\":0,\"b\":1,\"c\":2,\"d\":3,\"e\":4,\"f\":5,\"g\":6,\"h\":7,\"i\":8,\"j\":9,\"k\":10,\"l\":11,\"m\":12,\"n\":13,\"o\":14,\"p\":15,\"q\":16,\"r\":17,\"s\":18,\"t\":19,\"u\":20,\"v\":21,\"w\":22,\"x\":23,\"y\":24,\"z\":25}").expect("Invalid JSON given"));
    b.iter(|| { compile("a && b && c && d && e && f && g && h && i && j && k && l && m && n && o && p && q && r && s && t && u && v && w && x && y && z").unwrap().search(&data).ok() });
}



fn benchmarks_1_1_deep_ors_parse_lex(b: &mut Bencher) {
    b.iter(|| { parse("z || y || x || w || v || u || t || s || r || q || p || o || n || m || l || k || j || i || h || g || f || e || d || c || b || a").ok() });
}



fn benchmarks_1_1_deep_ors_interpret(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"a\":0,\"b\":1,\"c\":2,\"d\":3,\"e\":4,\"f\":5,\"g\":6,\"h\":7,\"i\":8,\"j\":9,\"k\":10,\"l\":11,\"m\":12,\"n\":13,\"o\":14,\"p\":15,\"q\":16,\"r\":17,\"s\":18,\"t\":19,\"u\":20,\"v\":21,\"w\":22,\"x\":23,\"y\":24,\"z\":25}").expect("Invalid JSON given"));
    let expr = compile("z || y || x || w || v || u || t || s || r || q || p || o || n || m || l || k || j || i || h || g || f || e || d || c || b || a").unwrap();
    b.iter(|| { expr.search(&data).ok() });
}



fn benchmarks_1_1_deep_ors_full(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"a\":0,\"b\":1,\"c\":2,\"d\":3,\"e\":4,\"f\":5,\"g\":6,\"h\":7,\"i\":8,\"j\":9,\"k\":10,\"l\":11,\"m\":12,\"n\":13,\"o\":14,\"p\":15,\"q\":16,\"r\":17,\"s\":18,\"t\":19,\"u\":20,\"v\":21,\"w\":22,\"x\":23,\"y\":24,\"z\":25}").expect("Invalid JSON given"));
    b.iter(|| { compile("z || y || x || w || v || u || t || s || r || q || p || o || n || m || l || k || j || i || h || g || f || e || d || c || b || a").unwrap().search(&data).ok() });
}



fn benchmarks_1_2_lots_of_summing_parse_lex(b: &mut Bencher) {
    b.iter(|| { parse("sum(z, y, x, w, v, u, t, s, r, q, p, o, n, m, l, k, j, i, h, g, f, e, d, c, b, a)").ok() });
}



fn benchmarks_1_2_lots_of_summing_interpret(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"a\":0,\"b\":1,\"c\":2,\"d\":3,\"e\":4,\"f\":5,\"g\":6,\"h\":7,\"i\":8,\"j\":9,\"k\":10,\"l\":11,\"m\":12,\"n\":13,\"o\":14,\"p\":15,\"q\":16,\"r\":17,\"s\":18,\"t\":19,\"u\":20,\"v\":21,\"w\":22,\"x\":23,\"y\":24,\"z\":25}").expect("Invalid JSON given"));
    let expr = compile("sum(z, y, x, w, v, u, t, s, r, q, p, o, n, m, l, k, j, i, h, g, f, e, d, c, b, a)").unwrap();
    b.iter(|| { expr.search(&data).ok() });
}



fn benchmarks_1_2_lots_of_summing_full(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"a\":0,\"b\":1,\"c\":2,\"d\":3,\"e\":4,\"f\":5,\"g\":6,\"h\":7,\"i\":8,\"j\":9,\"k\":10,\"l\":11,\"m\":12,\"n\":13,\"o\":14,\"p\":15,\"q\":16,\"r\":17,\"s\":18,\"t\":19,\"u\":20,\"v\":21,\"w\":22,\"x\":23,\"y\":24,\"z\":25}").expect("Invalid JSON given"));
    b.iter(|| { compile("sum(z, y, x, w, v, u, t, s, r, q, p, o, n, m, l, k, j, i, h, g, f, e, d, c, b, a)").unwrap().search(&data).ok() });
}



fn benchmarks_1_3_lots_of_multi_list_parse_lex(b: &mut Bencher) {
    b.iter(|| { parse("[z, y, x, w, v, u, t, s, r, q, p, o, n, m, l, k, j, i, h, g, f, e, d, c, b, a]").ok() });
}



fn benchmarks_1_3_lots_of_multi_list_interpret(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"a\":0,\"b\":1,\"c\":2,\"d\":3,\"e\":4,\"f\":5,\"g\":6,\"h\":7,\"i\":8,\"j\":9,\"k\":10,\"l\":11,\"m\":12,\"n\":13,\"o\":14,\"p\":15,\"q\":16,\"r\":17,\"s\":18,\"t\":19,\"u\":20,\"v\":21,\"w\":22,\"x\":23,\"y\":24,\"z\":25}").expect("Invalid JSON given"));
    let expr = compile("[z, y, x, w, v, u, t, s, r, q, p, o, n, m, l, k, j, i, h, g, f, e, d, c, b, a]").unwrap();
    b.iter(|| { expr.search(&data).ok() });
}



fn benchmarks_1_3_lots_of_multi_list_full(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"a\":0,\"b\":1,\"c\":2,\"d\":3,\"e\":4,\"f\":5,\"g\":6,\"h\":7,\"i\":8,\"j\":9,\"k\":10,\"l\":11,\"m\":12,\"n\":13,\"o\":14,\"p\":15,\"q\":16,\"r\":17,\"s\":18,\"t\":19,\"u\":20,\"v\":21,\"w\":22,\"x\":23,\"y\":24,\"z\":25}").expect("Invalid JSON given"));
    b.iter(|| { compile("[z, y, x, w, v, u, t, s, r, q, p, o, n, m, l, k, j, i, h, g, f, e, d, c, b, a]").unwrap().search(&data).ok() });
}



fn benchmarks_2_0_simple_field_parse_lex(b: &mut Bencher) {
    b.iter(|| { parse("a").ok() });
}



fn benchmarks_2_0_simple_field_interpret(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"long_name_for_a_field\":true,\"a\":{\"b\":{\"c\":{\"d\":{\"e\":{\"f\":{\"g\":{\"h\":{\"i\":{\"j\":{\"k\":{\"l\":{\"m\":{\"n\":{\"o\":{\"p\":true}}}}}}}}}}}}}}}}").expect("Invalid JSON given"));
    let expr = compile("a").unwrap();
    b.iter(|| { expr.search(&data).ok() });
}



fn benchmarks_2_0_simple_field_full(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"long_name_for_a_field\":true,\"a\":{\"b\":{\"c\":{\"d\":{\"e\":{\"f\":{\"g\":{\"h\":{\"i\":{\"j\":{\"k\":{\"l\":{\"m\":{\"n\":{\"o\":{\"p\":true}}}}}}}}}}}}}}}}").expect("Invalid JSON given"));
    b.iter(|| { compile("a").unwrap().search(&data).ok() });
}



fn benchmarks_2_1_simple_subexpression_parse_lex(b: &mut Bencher) {
    b.iter(|| { parse("a.b").ok() });
}



fn benchmarks_2_1_simple_subexpression_interpret(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"long_name_for_a_field\":true,\"a\":{\"b\":{\"c\":{\"d\":{\"e\":{\"f\":{\"g\":{\"h\":{\"i\":{\"j\":{\"k\":{\"l\":{\"m\":{\"n\":{\"o\":{\"p\":true}}}}}}}}}}}}}}}}").expect("Invalid JSON given"));
    let expr = compile("a.b").unwrap();
    b.iter(|| { expr.search(&data).ok() });
}



fn benchmarks_2_1_simple_subexpression_full(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"long_name_for_a_field\":true,\"a\":{\"b\":{\"c\":{\"d\":{\"e\":{\"f\":{\"g\":{\"h\":{\"i\":{\"j\":{\"k\":{\"l\":{\"m\":{\"n\":{\"o\":{\"p\":true}}}}}}}}}}}}}}}}").expect("Invalid JSON given"));
    b.iter(|| { compile("a.b").unwrap().search(&data).ok() });
}



fn benchmarks_2_2_deep_field_selection_parse_lex(b: &mut Bencher) {
    b.iter(|| { parse("a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s").ok() });
}



fn benchmarks_2_2_deep_field_selection_interpret(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"long_name_for_a_field\":true,\"a\":{\"b\":{\"c\":{\"d\":{\"e\":{\"f\":{\"g\":{\"h\":{\"i\":{\"j\":{\"k\":{\"l\":{\"m\":{\"n\":{\"o\":{\"p\":true}}}}}}}}}}}}}}}}").expect("Invalid JSON given"));
    let expr = compile("a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s").unwrap();
    b.iter(|| { expr.search(&data).ok() });
}



fn benchmarks_2_2_deep_field_selection_full(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"long_name_for_a_field\":true,\"a\":{\"b\":{\"c\":{\"d\":{\"e\":{\"f\":{\"g\":{\"h\":{\"i\":{\"j\":{\"k\":{\"l\":{\"m\":{\"n\":{\"o\":{\"p\":true}}}}}}}}}}}}}}}}").expect("Invalid JSON given"));
    b.iter(|| { compile("a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s").unwrap().search(&data).ok() });
}



fn benchmarks_2_3_simple_or_parse_lex(b: &mut Bencher) {
    b.iter(|| { parse("not_there || a").ok() });
}



fn benchmarks_2_3_simple_or_interpret(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"long_name_for_a_field\":true,\"a\":{\"b\":{\"c\":{\"d\":{\"e\":{\"f\":{\"g\":{\"h\":{\"i\":{\"j\":{\"k\":{\"l\":{\"m\":{\"n\":{\"o\":{\"p\":true}}}}}}}}}}}}}}}}").expect("Invalid JSON given"));
    let expr = compile("not_there || a").unwrap();
    b.iter(|| { expr.search(&data).ok() });
}



fn benchmarks_2_3_simple_or_full(b: &mut Bencher) {
    let data = Rcvar::new(Variable::from_json("{\"long_name_for_a_field\":true,\"a\":{\"b\":{\"c\":{\"d\":{\"e\":{\"f\":{\"g\":{\"h\":{\"i\":{\"j\":{\"k\":{\"l\":{\"m\":{\"n\":{\"o\":{\"p\":true}}}}}}}}}}}}}}}}").expect("Invalid JSON given"));
    b.iter(|| { compile("not_there || a").unwrap().search(&data).ok() });
}


benchmark_group!(benches, benchmarks_0_0_field_50_parse_lex, benchmarks_0_1_pipe_50_parse_lex, benchmarks_0_2_index_50_parse_lex, benchmarks_0_3_long_raw_string_literal_parse_lex, benchmarks_0_4_deep_projection_104_parse_lex, benchmarks_0_5_filter_projection_parse_lex, benchmarks_1_0_deep_ands_parse_lex, benchmarks_1_0_deep_ands_interpret, benchmarks_1_0_deep_ands_full, benchmarks_1_1_deep_ors_parse_lex, benchmarks_1_1_deep_ors_interpret, benchmarks_1_1_deep_ors_full, benchmarks_1_2_lots_of_summing_parse_lex, benchmarks_1_2_lots_of_summing_interpret, benchmarks_1_2_lots_of_summing_full, benchmarks_1_3_lots_of_multi_list_parse_lex, benchmarks_1_3_lots_of_multi_list_interpret, benchmarks_1_3_lots_of_multi_list_full, benchmarks_2_0_simple_field_parse_lex, benchmarks_2_0_simple_field_interpret, benchmarks_2_0_simple_field_full, benchmarks_2_1_simple_subexpression_parse_lex, benchmarks_2_1_simple_subexpression_interpret, benchmarks_2_1_simple_subexpression_full, benchmarks_2_2_deep_field_selection_parse_lex, benchmarks_2_2_deep_field_selection_interpret, benchmarks_2_2_deep_field_selection_full, benchmarks_2_3_simple_or_parse_lex, benchmarks_2_3_simple_or_interpret, benchmarks_2_3_simple_or_full);
benchmark_main!(benches);
