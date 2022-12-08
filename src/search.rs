pub fn search_enumerate<'a: 'b, 'b, T, I>(
    query: &str,
    items: &'a [T],
    f: impl Fn(&'b T) -> I,
) -> impl ExactSizeIterator<Item = (usize, &'a T)>
where
    T: 'b,
    I: IntoIterator<Item = (isize, &'b str)> + 'b,
{
    let mut results = Vec::with_capacity(items.len());

    for (i, item) in items.iter().enumerate() {
        for (offset, string) in f(item) {
            let Some(best) = sublime_fuzzy::best_match(query, string) else {
                continue;
            };

            results.push((best.score() + offset, i, item));
            break;
        }
    }

    results.sort_by(|a, b| a.0.cmp(&b.0).reverse());
    results.into_iter().map(|(_, i, item)| (i, item))
}
